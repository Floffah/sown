use super::*;
use protocol::{
    Message,
    v1::{
        ClientStreamHeader, ControlClientHello, ControlClientJoin, ControlEnvelope, EventEnvelope,
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
    world: Arc<Mutex<world::World>>,
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
        let source = project::world::parse(WORLD)?;
        let ecs = Ecs::from_source(&source).map_err(anyhow::Error::msg)?;
        let world = Arc::new(Mutex::new(world::World::new(ecs)));
        let task = tokio::spawn(serve(server.clone(), world.clone()));
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
            world,
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
            username: None,
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

fn player_count(server: &TestServer) -> usize {
    let mut world = server.world.lock().unwrap();
    world.ecs.world.query_filtered::<engine::ecs::bevy_ecs::prelude::Entity,
        engine::ecs::bevy_ecs::prelude::With<engine::ecs::player::Player>>()
        .iter(&world.ecs.world).count()
}

#[tokio::test]
async fn joins_two_players_and_broadcasts_creation_and_removal() -> Result<()> {
    tokio::time::timeout(Duration::from_secs(10), async {
        let server = TestServer::new()?;
        let first = server.connect().await?;
        let (capture, captured) = networking::inspection::capture_channel();
        let mut a = networking::quic::handshake(&first, Some(&capture)).await?;
        let second = server.connect().await?;
        let mut b = networking::quic::handshake(&second, None).await?;
        assert_ne!(a.peer_id, b.peer_id);
        assert_eq!(player_count(&server), 0);
        let initial_a = networking::quic::join(&mut a).await?;
        assert_eq!(initial_a.peers.len(), 1);
        assert_eq!(initial_a.entities.len(), 1);
        let packets: Vec<_> = std::iter::from_fn(|| captured.try_recv()).collect();
        assert_eq!(packets.len(), 8);
        assert_eq!(captured.dropped(), 0);
        assert_eq!(
            packets[1].direction,
            networking::inspection::Direction::Sent
        );
        assert_eq!(
            packets[1].json()?["clientHello"]["protocolVersion"].as_u64(),
            Some(1)
        );
        assert_eq!(
            packets[2].direction,
            networking::inspection::Direction::Received
        );
        assert_eq!(
            packets[5].json()?["serverJoined"]["playerEntityId"].as_str(),
            Some(initial_a.player_entity_id.to_string().as_str())
        );
        assert_eq!(packets[7].channel, networking::inspection::Channel::Events);
        let initial_b = networking::quic::join(&mut b).await?;
        assert_eq!(initial_b.peers.len(), 2);
        assert_eq!(initial_b.entities.len(), 2);
        assert_ne!(initial_a.player_entity_id, initial_b.player_entity_id);
        assert_eq!(player_count(&server), 2);
        let admitted: EventEnvelope =
            networking::quic::read_message(&mut a.events_recv, None).await?;
        assert!(
            matches!(admitted.payload, Some(event_envelope::Payload::PeerAdmitted(peer))
            if peer.snapshot.as_ref().unwrap().peer_id == b.peer_id)
        );
        let created: EventEnvelope =
            networking::quic::read_message(&mut a.events_recv, None).await?;
        assert!(
            matches!(created.payload, Some(event_envelope::Payload::EntityCreated(entity))
            if entity.snapshot.unwrap().entity_id == initial_b.player_entity_id)
        );
        second.close(0_u8.into(), b"Leaving");
        let removed: EventEnvelope =
            networking::quic::read_message(&mut a.events_recv, None).await?;
        assert!(
            matches!(removed.payload, Some(event_envelope::Payload::EntityRemoved(entity))
            if entity.entity_id == initial_b.player_entity_id)
        );
        let removed: EventEnvelope =
            networking::quic::read_message(&mut a.events_recv, None).await?;
        assert!(
            matches!(removed.payload, Some(event_envelope::Payload::PeerRemoved(peer))
            if peer.peer_id == b.peer_id)
        );
        assert_eq!(player_count(&server), 1);
        networking::quic::write_message(
            &mut a.control_send,
            &ControlEnvelope {
                payload: Some(control_envelope::Payload::ClientJoin(ControlClientJoin {})),
            },
            None,
        )
        .await?;
        rejection(&first, "Unexpected Control message after Joined").await?;
        assert_eq!(player_count(&server), 0);
        server.shutdown().await
    })
    .await?
}

#[tokio::test]
async fn full_capture_queue_does_not_interrupt_join() -> Result<()> {
    tokio::time::timeout(Duration::from_secs(10), async {
        let server = TestServer::new()?;
        let connection = server.connect().await?;
        let (capture, captured) = networking::inspection::capture_channel();
        let context = capture.context(1, networking::inspection::Channel::Control);
        let body = hello(1).encode_to_vec();
        for _ in 0..300 {
            context.record::<ControlEnvelope>(
                networking::inspection::Direction::Sent,
                &body,
                body.len() + 1,
            );
        }
        let dropped_before_join = captured.dropped();
        assert!(dropped_before_join > 0);
        let mut streams = networking::quic::handshake(&connection, Some(&capture)).await?;
        let world = networking::quic::join(&mut streams).await?;
        assert_eq!(world.entities.len(), 1);
        assert_eq!(player_count(&server), 1);
        assert_eq!(captured.dropped(), dropped_before_join + 8);
        server.shutdown().await
    })
    .await?
}

#[tokio::test]
async fn accepts_join_before_events_and_rejects_client_events() -> Result<()> {
    tokio::time::timeout(Duration::from_secs(10), async {
        let server = TestServer::new()?;
        let connection = server.connect().await?;
        let (mut control_send, mut control_recv) = connection.open_bi().await?;
        networking::quic::write_message(
            &mut control_send,
            &ClientStreamHeader {
                stream_id: StreamId::Control as i32,
            },
            None,
        )
        .await?;
        networking::quic::write_message(&mut control_send, &hello(1), None).await?;
        let welcome: ControlEnvelope =
            networking::quic::read_message(&mut control_recv, None).await?;
        assert!(matches!(
            welcome.payload,
            Some(control_envelope::Payload::ServerWelcome(_))
        ));
        networking::quic::write_message(
            &mut control_send,
            &ControlEnvelope {
                payload: Some(control_envelope::Payload::ClientJoin(ControlClientJoin {})),
            },
            None,
        )
        .await?;
        assert_eq!(player_count(&server), 0);
        let (mut events_send, mut events_recv) = connection.open_bi().await?;
        networking::quic::write_message(
            &mut events_send,
            &ClientStreamHeader {
                stream_id: StreamId::Events as i32,
            },
            None,
        )
        .await?;
        let joined: ControlEnvelope =
            networking::quic::read_message(&mut control_recv, None).await?;
        assert!(matches!(
            joined.payload,
            Some(control_envelope::Payload::ServerJoined(_))
        ));
        let _: EventEnvelope = networking::quic::read_message(&mut events_recv, None).await?;
        let _: EventEnvelope = networking::quic::read_message(&mut events_recv, None).await?;
        events_send.write_all(&[1, 0xff]).await?;
        rejection(&connection, "Invalid protobuf message").await?;
        assert_eq!(player_count(&server), 0);
        server.shutdown().await
    })
    .await?
}

#[tokio::test]
async fn rejects_invalid_headers_handshakes_and_frames() -> Result<()> {
    tokio::time::timeout(Duration::from_secs(15), async {
        let server = TestServer::new()?;
        let control = ClientStreamHeader {
            stream_id: StreamId::Control as i32,
        }
        .encode_length_delimited_to_vec();
        let mut wrong_version = control.clone();
        wrong_version.extend(hello(2).encode_length_delimited_to_vec());
        let mut wrong_payload = control.clone();
        wrong_payload.extend(ControlEnvelope { payload: None }.encode_length_delimited_to_vec());
        let cases = [
            (
                ClientStreamHeader { stream_id: 99 }.encode_length_delimited_to_vec(),
                "Unknown stream channel",
            ),
            (
                ClientStreamHeader {
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
        let _: ControlEnvelope = networking::quic::read_message(&mut recv, None).await?;
        let (mut duplicate, _recv) = connection.open_bi().await?;
        networking::quic::write_message(
            &mut duplicate,
            &ClientStreamHeader {
                stream_id: StreamId::Control as i32,
            },
            None,
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
