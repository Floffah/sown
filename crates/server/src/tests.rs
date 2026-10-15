use super::*;
use protocol::{
    Message,
    v1::{
        ControlClientHello, ControlEnvelope, EventEnvelope, EventPlaceholderMessage, StreamHeader,
        StreamId, control_envelope, event_envelope,
    },
};
use quinn::{Connection, crypto::rustls::QuicClientConfig};
use rustls::pki_types::PrivatePkcs8KeyDer;
use std::time::Duration;

struct TestServer {
    server: Endpoint,
    client: Endpoint,
    task: tokio::task::JoinHandle<Result<()>>,
}

impl TestServer {
    fn new() -> Result<Self> {
        let certificate = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
        let key = PrivatePkcs8KeyDer::from(certificate.signing_key.serialize_der());
        let server = create_endpoint(
            "127.0.0.1:0".parse()?,
            certificate.cert.der().clone(),
            key.into(),
        )?;
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
        Ok(Self {
            server,
            client,
            task,
        })
    }

    async fn connect(&self) -> Result<Connection> {
        Ok(self
            .client
            .connect(self.server.local_addr()?, "localhost")?
            .await?)
    }

    async fn shutdown(self) -> Result<()> {
        self.server.close(0_u8.into(), b"Test complete");
        self.task.await??;
        self.client.wait_idle().await;
        Ok(())
    }
}

fn hello(version: u32) -> ControlEnvelope {
    ControlEnvelope {
        payload: Some(control_envelope::Payload::ClientHello(ControlClientHello {
            protocol_version: version,
        })),
    }
}

async fn rejection(connection: &Connection, expected_reason: &str) -> Result<()> {
    let error = tokio::time::timeout(Duration::from_secs(3), connection.closed()).await?;
    let quinn::ConnectionError::ApplicationClosed(close) = error else {
        anyhow::bail!("Expected an application rejection, got {error}");
    };
    assert_eq!(close.error_code, 1_u8.into());
    assert!(String::from_utf8_lossy(&close.reason).contains(expected_reason));
    Ok(())
}

#[tokio::test]
async fn welcomes_client_and_reads_events_while_control_is_idle() -> Result<()> {
    tokio::time::timeout(Duration::from_secs(10), async {
        let server = TestServer::new()?;
        let connection = server.connect().await?;
        let mut streams = networking::quic::join(&connection).await?;
        networking::quic::write_message(
            &mut streams.events_send,
            &EventEnvelope {
                payload: Some(event_envelope::Payload::Placeholder(
                    EventPlaceholderMessage {
                        protocol_version: 1,
                    },
                )),
            },
        )
        .await?;
        // The invalid next event must be read despite the idle Control stream.
        streams.events_send.write_all(&[1, 0xff]).await?;
        rejection(&connection, "Invalid protobuf message").await?;
        server.shutdown().await
    })
    .await?
}

#[tokio::test]
async fn rejects_invalid_headers_handshakes_and_frames() -> Result<()> {
    tokio::time::timeout(Duration::from_secs(15), async {
        let server = TestServer::new()?;
        let control = StreamHeader {
            stream_id: StreamId::Control as i32,
        }
        .encode_length_delimited_to_vec();
        let mut wrong_version = control.clone();
        wrong_version.extend(hello(2).encode_length_delimited_to_vec());
        let mut wrong_payload = control.clone();
        wrong_payload.extend(ControlEnvelope { payload: None }.encode_length_delimited_to_vec());
        let cases = [
            (
                StreamHeader { stream_id: 99 }.encode_length_delimited_to_vec(),
                "Unknown stream channel",
            ),
            (
                StreamHeader {
                    stream_id: StreamId::Events as i32,
                }
                .encode_length_delimited_to_vec(),
                "Expected Control stream",
            ),
            (wrong_version, "Unsupported protocol version"),
            (wrong_payload, "Expected ClientHello"),
            (vec![0x81, 0x80, 0x04], "Message exceeds 64 KiB limit"),
            (vec![0x80; 10], "Invalid message length varint"),
            (vec![0x80], "Failed to read message length"),
            (vec![2, 0x08], "Failed to read message body"),
        ];
        let mut second_control = control;
        second_control.extend(hello(1).encode_length_delimited_to_vec());
        for (bytes, reason) in cases {
            let connection = server.connect().await?;
            let (mut send, _recv) = connection.open_bi().await?;
            send.write_all(&bytes).await?;
            send.finish()?;
            rejection(&connection, reason).await?;
        }
        let connection = server.connect().await?;
        let (mut send, mut recv) = connection.open_bi().await?;
        send.write_all(&second_control).await?;
        let _: ControlEnvelope = networking::quic::read_message(&mut recv).await?;
        let (mut duplicate, _recv) = connection.open_bi().await?;
        networking::quic::write_message(
            &mut duplicate,
            &StreamHeader {
                stream_id: StreamId::Control as i32,
            },
        )
        .await?;
        rejection(&connection, "Expected Events stream").await?;
        server.shutdown().await
    })
    .await?
}

#[tokio::test]
async fn rejects_client_that_never_opens_control() -> Result<()> {
    tokio::time::timeout(Duration::from_secs(15), async {
        let server = TestServer::new()?;
        let connection = server.connect().await?;
        let error = tokio::time::timeout(Duration::from_secs(12), connection.closed()).await?;
        let quinn::ConnectionError::ApplicationClosed(close) = error else {
            anyhow::bail!("Expected handshake timeout, got {error}");
        };
        assert!(String::from_utf8_lossy(&close.reason).contains("Control handshake timed out"));
        server.shutdown().await
    })
    .await?
}
