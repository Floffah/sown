use std::{sync::Mutex, time::Duration};

use anyhow::{Context, Result, bail, ensure};
use networking::{
    PROTOCOL_VERSION,
    quic::{read_message, write_control, write_message},
};
use protocol::v1::{
    ClientStreamHeader, ControlEnvelope, ControlServerJoined, ControlServerWelcome, EventEnvelope,
    StreamId, control_envelope,
};
use quinn::{Connection, RecvStream, SendStream};
use tokio::sync::mpsc;

use crate::world::World;

const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

struct PeerGuard<'a> {
    world: &'a Mutex<World>,
    peer_id: u32,
}

impl Drop for PeerGuard<'_> {
    fn drop(&mut self) {
        self.world.lock().unwrap().remove(self.peer_id);
    }
}

pub(super) async fn run(connection: &Connection, world: &Mutex<World>) -> Result<()> {
    let peer_id = world.lock().unwrap().allocate_peer_id()?;
    let _guard = PeerGuard { world, peer_id };
    let (mut control_send, mut control_recv, username) =
        tokio::time::timeout(HANDSHAKE_TIMEOUT, handshake(connection, peer_id))
            .await
            .context("Control handshake timed out")??;
    println!("Handshake complete: peer {peer_id}");

    let ((), (events_send, events_recv)) =
        tokio::try_join!(read_join(&mut control_recv), accept_events(connection),)?;
    let (player_entity_id, events) =
        world
            .lock()
            .unwrap()
            .admit(peer_id, username, connection.clone())?;
    let joined = ControlServerJoined { player_entity_id };
    write_control(
        &mut control_send,
        control_envelope::Payload::ServerJoined(joined),
    )
    .await?;

    println!("Client joined: peer {peer_id}, player entity {player_entity_id}");

    tokio::try_join!(
        consume_control(control_send, control_recv),
        consume_events(events_recv),
        send_events(events_send, events),
    )?;
    Ok(())
}

async fn handshake(
    connection: &Connection,
    peer_id: u32,
) -> Result<(SendStream, RecvStream, Option<String>)> {
    let (mut send, mut recv) = connection.accept_bi().await?;
    read_header(&mut recv, StreamId::Control).await?;

    let message: ControlEnvelope = read_message(&mut recv).await?;
    let Some(control_envelope::Payload::ClientHello(hello)) = message.payload else {
        bail!("Expected ClientHello on Control");
    };
    ensure!(
        hello.protocol_version == PROTOCOL_VERSION,
        "Unsupported protocol version: {}",
        hello.protocol_version
    );

    let welcome = ControlServerWelcome {
        protocol_version: PROTOCOL_VERSION,
        peer_id,
    };
    write_control(&mut send, control_envelope::Payload::ServerWelcome(welcome)).await?;
    Ok((send, recv, hello.username))
}

async fn read_header(recv: &mut RecvStream, expected: StreamId) -> Result<()> {
    let header: ClientStreamHeader = read_message(recv).await?;
    let channel = StreamId::try_from(header.stream_id).context("Unknown stream channel")?;
    ensure!(
        channel == expected,
        "Expected {expected:?} stream, received {channel:?}"
    );
    Ok(())
}

async fn read_join(recv: &mut RecvStream) -> Result<()> {
    let message: ControlEnvelope = read_message(recv).await?;
    ensure!(
        matches!(
            message.payload,
            Some(control_envelope::Payload::ClientJoin(_))
        ),
        "Expected ClientJoin on Control"
    );
    Ok(())
}

async fn accept_events(connection: &Connection) -> Result<(SendStream, RecvStream)> {
    let (send, mut recv) = connection.accept_bi().await?;
    tokio::time::timeout(HANDSHAKE_TIMEOUT, read_header(&mut recv, StreamId::Events))
        .await
        .context("Events stream header timed out")??;
    Ok((send, recv))
}

async fn consume_control(_send: SendStream, mut recv: RecvStream) -> Result<()> {
    let _: ControlEnvelope = read_message(&mut recv).await?;
    bail!("Unexpected Control message after Joined")
}

async fn consume_events(mut recv: RecvStream) -> Result<()> {
    let _: EventEnvelope = read_message(&mut recv).await?;
    bail!("Unexpected client Events message")
}

async fn send_events(
    mut send: SendStream,
    mut events: mpsc::Receiver<EventEnvelope>,
) -> Result<()> {
    while let Some(event) = events.recv().await {
        write_message(&mut send, &event).await?;
    }
    bail!("Events queue closed")
}
