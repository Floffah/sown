use std::{net::SocketAddr, path::Path, sync::Arc};

use anyhow::{Context, Result};
use engine::ecs::Ecs;
use quinn::{Endpoint, crypto::rustls::QuicServerConfig, rustls};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

mod tls;

const WORLD: &str = include_str!("../../../fixtures/minimal-room/worlds/main.world");
const LISTEN_ADDRESS: &str = "127.0.0.1:50420";
const ALPN: &[u8] = b"sown/1";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let source = project::world::parse(WORLD).context("Failed to parse the embedded world")?;
    let _ecs = Ecs::from_source(&source)
        .map_err(anyhow::Error::msg)
        .context("Failed to build the server ECS")?;
    println!("Loaded world: {}", source.name);

    let (certificate, key) = tls::load_or_generate(Path::new(".sown"))?;
    let endpoint = create_endpoint(LISTEN_ADDRESS.parse()?, certificate, key)?;
    println!("QUIC listening on {}", endpoint.local_addr()?);
    serve(endpoint).await
}

fn create_endpoint(
    address: SocketAddr,
    certificate: CertificateDer<'static>,
    key: PrivateKeyDer<'static>,
) -> Result<Endpoint> {
    let mut tls = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![certificate], key)?;
    tls.alpn_protocols = vec![ALPN.to_vec()];
    let config = quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(tls)?));
    Endpoint::server(config, address).context("Failed to bind the QUIC endpoint")
}

async fn serve(endpoint: Endpoint) -> Result<()> {
    loop {
        tokio::select! {
            result = tokio::signal::ctrl_c() => {
                result.context("Failed to listen for Ctrl-C")?;
                break;
            }
            incoming = endpoint.accept() => {
                let Some(incoming) = incoming else { break };
                tokio::spawn(async move {
                    match incoming.await {
                        Ok(connection) => {
                            println!("QUIC connected: {}", connection.remote_address());
                            let reason = connection.closed().await;
                            println!("QUIC disconnected: {reason}");
                        }
                        Err(error) => eprintln!("QUIC connection failed: {error}"),
                    }
                });
            }
        }
    }
    endpoint.close(0_u8.into(), b"Server shutting down");
    endpoint.wait_idle().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use quinn::crypto::rustls::QuicClientConfig;
    use rustls::pki_types::PrivatePkcs8KeyDer;
    use std::time::Duration;

    #[tokio::test]
    async fn accepts_connections_with_a_trusted_localhost_certificate() -> Result<()> {
        tokio::time::timeout(Duration::from_secs(5), async {
            let certificate = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
            let key = PrivatePkcs8KeyDer::from(certificate.signing_key.serialize_der());
            let server = create_endpoint(
                "127.0.0.1:0".parse()?,
                certificate.cert.der().clone(),
                key.into(),
            )?;
            let address = server.local_addr()?;
            let task = tokio::spawn(serve(server.clone()));

            let mut roots = rustls::RootCertStore::empty();
            roots.add(certificate.cert.der().clone())?;
            let mut tls = rustls::ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth();
            tls.alpn_protocols = vec![ALPN.to_vec()];
            let config = quinn::ClientConfig::new(Arc::new(QuicClientConfig::try_from(tls)?));
            let mut client = Endpoint::client("127.0.0.1:0".parse()?)?;
            client.set_default_client_config(config);

            for _ in 0..2 {
                let connection = client.connect(address, "localhost")?.await?;
                assert_eq!(connection.remote_address(), address);
                connection.close(0_u8.into(), b"Test complete");
            }
            server.close(0_u8.into(), b"Test complete");
            task.await??;
            client.wait_idle().await;
            Ok(())
        })
        .await?
    }
}
