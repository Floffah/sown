use std::{
    net::SocketAddr,
    path::Path,
    sync::mpsc,
    thread::{self, JoinHandle},
};

use anyhow::{Context, Result};
use quinn::{Endpoint, rustls::pki_types::CertificateDer};

use crate::{Server, serve};

/// Owns a loopback server running on a dedicated thread. Dropping it stops and joins the thread.
pub struct EmbeddedServer {
    endpoint: Endpoint,
    certificate: CertificateDer<'static>,
    thread: Option<JoinHandle<Result<()>>>,
}

impl EmbeddedServer {
    /// Builds an independent world and binds an OS-assigned loopback port before returning.
    ///
    /// # Errors
    /// Returns thread, runtime, world-loading, TLS, or endpoint startup errors.
    pub fn start(source: &str, state_directory: &Path) -> Result<Self> {
        let source = source.to_owned();
        let state_directory = state_directory.to_owned();
        let (ready, receiver) = mpsc::sync_channel(1);
        let thread = thread::Builder::new()
            .name("sown-server".into())
            .spawn(move || {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(async {
                    let server = Server::bind(&source, "127.0.0.1:0".parse()?, &state_directory)?;
                    ready
                        .send((server.endpoint.clone(), server.certificate))
                        .map_err(|_| anyhow::anyhow!("Embedded server startup cancelled"))?;
                    serve(server.endpoint, server.world).await
                })
            })
            .context("Failed to start the embedded server thread")?;
        if let Ok((endpoint, certificate)) = receiver.recv() {
            Ok(Self {
                endpoint,
                certificate,
                thread: Some(thread),
            })
        } else {
            join_thread(thread)?;
            anyhow::bail!("Embedded server stopped before startup completed")
        }
    }

    /// # Errors
    /// Returns an error if the endpoint's bound address cannot be read.
    pub fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.endpoint.local_addr()?)
    }

    /// The exact certificate to trust for this server's localhost connection.
    #[must_use]
    pub fn certificate(&self) -> CertificateDer<'static> {
        self.certificate.clone()
    }

    /// Closes connections and waits for the server thread to finish.
    ///
    /// # Errors
    /// Returns the server's runtime error or reports a thread panic.
    pub fn shutdown(mut self) -> Result<()> {
        self.stop()
    }

    fn stop(&mut self) -> Result<()> {
        self.endpoint.close(0_u8.into(), b"Local session ended");
        if let Some(thread) = self.thread.take() {
            join_thread(thread)?;
        }
        Ok(())
    }
}

impl Drop for EmbeddedServer {
    fn drop(&mut self) {
        if let Err(error) = self.stop() {
            eprintln!("Embedded server shutdown failed: {error:#}");
        }
    }
}

fn join_thread(thread: JoinHandle<Result<()>>) -> Result<()> {
    thread
        .join()
        .map_err(|_| anyhow::anyhow!("Embedded server thread panicked"))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_errors_propagate_and_drop_releases_the_port() -> Result<()> {
        let directory = std::env::temp_dir().join(format!(
            "sown-embedded-server-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos(),
        ));
        assert!(EmbeddedServer::start("invalid world", &directory).is_err());
        assert!(!directory.exists());
        let server = EmbeddedServer::start(crate::WORLD, &directory)?;
        let address = server.local_addr()?;
        drop(server);
        let _socket = std::net::UdpSocket::bind(address)?;
        std::fs::remove_dir_all(directory)?;
        Ok(())
    }
}
