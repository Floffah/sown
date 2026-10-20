use std::{
    net::SocketAddr,
    sync::{Arc, mpsc::SyncSender},
    time::Duration,
};

use anyhow::{Context, Result};
use networking::{
    ALPN,
    quic::{handshake, join, read_message},
};
use protocol::v1::ControlEnvelope;
use quinn::{Connection, Endpoint, crypto::rustls::QuicClientConfig, rustls};
use rustls::pki_types::{CertificateDer, pem::PemObject};

use crate::replication::Update;

const SERVER_ADDRESS: &str = "127.0.0.1:50420";
const CERTIFICATE_PATH: &str = ".sown/server-tls.pem";

pub(super) async fn run(
    updates: SyncSender<Update>,
    world_ready: tokio::sync::oneshot::Receiver<()>,
) -> Result<()> {
    let certificate = CertificateDer::from_pem_file(CERTIFICATE_PATH).with_context(|| {
        format!("Failed to load {CERTIFICATE_PATH}: run the server from the same working directory first")
    })?;
    println!("QUIC connecting to {SERVER_ADDRESS}");
    let (endpoint, connection) = connect(SERVER_ADDRESS.parse()?, certificate).await?;
    println!("QUIC connected to {}", connection.remote_address());
    let result = run_session(&connection, &updates, world_ready).await;
    if result.is_err() {
        connection.close(1_u8.into(), b"Client session failed");
    }
    drop(updates);
    endpoint.wait_idle().await;
    result
}

async fn run_session(
    connection: &Connection,
    updates: &SyncSender<Update>,
    world_ready: tokio::sync::oneshot::Receiver<()>,
) -> Result<()> {
    let mut streams = tokio::time::timeout(Duration::from_secs(10), handshake(connection))
        .await
        .context("Application handshake timed out")??;
    println!("Handshake complete: peer {}", streams.peer_id);

    world_ready
        .await
        .context("Client closed before its world was ready")?;

    let world = tokio::time::timeout(Duration::from_secs(10), join(&mut streams))
        .await
        .context("World join timed out")??;
    updates.try_send(Update::Joined {
        world,
        connection: connection.clone(),
    })?;

    tokio::select! {
        reason = connection.closed() => println!("QUIC disconnected: {reason}"),
        response = read_message::<ControlEnvelope>(&mut streams.control_recv) => {
            let _ = response?;
            anyhow::bail!("Unexpected Control message during Play");
        }
        result = receive_events(&mut streams.events_recv, updates) => result?,
    }
    Ok(())
}

async fn receive_events(recv: &mut quinn::RecvStream, updates: &SyncSender<Update>) -> Result<()> {
    loop {
        let event = read_message(recv).await?;
        updates.try_send(Update::Event(event))?;
    }
}

async fn connect(
    address: SocketAddr,
    certificate: CertificateDer<'static>,
) -> Result<(Endpoint, Connection)> {
    let mut roots = rustls::RootCertStore::empty();
    roots.add(certificate)?;
    let mut tls = rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    tls.alpn_protocols = vec![ALPN.to_vec()];
    let mut config = quinn::ClientConfig::new(Arc::new(QuicClientConfig::try_from(tls)?));
    let mut transport = quinn::TransportConfig::default();
    transport.keep_alive_interval(Some(Duration::from_secs(5)));
    config.transport_config(Arc::new(transport));
    let mut endpoint = Endpoint::client("127.0.0.1:0".parse()?)?;
    endpoint.set_default_client_config(config);
    let connection = tokio::time::timeout(
        Duration::from_secs(5),
        endpoint.connect(address, "localhost")?,
    )
    .await
    .context("QUIC connection timed out")??;
    Ok((endpoint, connection))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustls::pki_types::PrivatePkcs8KeyDer;

    #[tokio::test]
    async fn idle_connection_stays_alive_and_rejects_an_untrusted_certificate() -> Result<()> {
        tokio::time::timeout(Duration::from_secs(15), async {
            let certificate = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
            let key = PrivatePkcs8KeyDer::from(certificate.signing_key.serialize_der());
            let mut tls = rustls::ServerConfig::builder()
                .with_no_client_auth()
                .with_single_cert(vec![certificate.cert.der().clone()], key.into())?;
            tls.alpn_protocols = vec![ALPN.to_vec()];
            let mut config = quinn::ServerConfig::with_crypto(Arc::new(
                quinn::crypto::rustls::QuicServerConfig::try_from(tls)?,
            ));
            let mut transport = quinn::TransportConfig::default();
            transport.max_idle_timeout(Some(Duration::from_secs(6).try_into()?));
            config.transport_config(Arc::new(transport));
            let server = Endpoint::server(config, "127.0.0.1:0".parse()?)?;
            let address = server.local_addr()?;
            let accept = async {
                server
                    .accept()
                    .await
                    .context("Server stopped")?
                    .await
                    .context("Server handshake failed")
            };
            let ((client, connection), server_connection) =
                tokio::try_join!(connect(address, certificate.cert.der().clone()), accept,)?;
            assert_eq!(connection.remote_address(), address);
            assert!(
                tokio::time::timeout(Duration::from_millis(100), server_connection.accept_bi())
                    .await
                    .is_err()
            );
            tokio::time::sleep(Duration::from_secs(7)).await;
            assert!(connection.close_reason().is_none());
            assert!(server_connection.close_reason().is_none());

            let other = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
            let (rejected, _) = tokio::join!(connect(address, other.cert.der().clone()), async {
                server
                    .accept()
                    .await
                    .context("Server stopped")?
                    .await
                    .context("Server handshake failed")
            },);
            assert!(rejected.is_err());
            connection.close(0_u8.into(), b"Test complete");
            server.close(0_u8.into(), b"Test complete");
            client.wait_idle().await;
            server.wait_idle().await;
            Ok(())
        })
        .await?
    }
}
