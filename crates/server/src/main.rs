use std::{net::SocketAddr, path::Path, sync::Arc};

use anyhow::{Context, Result};
use engine::ecs::Ecs;
use networking::ALPN;
use quinn::{Endpoint, crypto::rustls::QuicServerConfig, rustls};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

mod session;
mod tls;

const WORLD: &str = include_str!("../../../fixtures/minimal-room/worlds/main.world");
const LISTEN_ADDRESS: &str = "127.0.0.1:50420";

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
    #[cfg(debug_assertions)]
    {
        // for letting wireshark decrypt us in dev
        tls.key_log = Arc::new(rustls::KeyLogFile::new());
    }
    let mut config = quinn::ServerConfig::with_crypto(Arc::new(QuicServerConfig::try_from(tls)?));
    let mut transport = quinn::TransportConfig::default();
    transport.max_concurrent_bidi_streams(2_u8.into());
    transport.max_concurrent_uni_streams(0_u8.into());
    config.transport_config(Arc::new(transport));
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
                            tokio::select! {
                                reason = connection.closed() => {
                                    println!("QUIC disconnected: {reason}");
                                }
                                result = session::run(&connection) => {
                                    if let Err(error) = result {
                                        if let Some(reason) = connection.close_reason() {
                                            println!("QUIC disconnected: {reason}");
                                        } else {
                                            eprintln!("Session rejected: {error}");
                                            connection.close(1_u8.into(), error.to_string().as_bytes());
                                        }
                                    }
                                }
                            }
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
mod tests;
