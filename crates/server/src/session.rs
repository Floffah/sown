use std::time::Duration;

use anyhow::{Context, Result, bail, ensure};
use networking::{
    PROTOCOL_VERSION,
    quic::{read_message, write_message},
};
use protocol::v1::{
    ControlEnvelope, ControlServerWelcome, EventEnvelope, StreamHeader, StreamId, control_envelope,
};
use quinn::{Connection, RecvStream, SendStream};

const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

pub(super) async fn run(connection: &Connection) -> Result<()> {
    let (control_send, control_recv) = tokio::time::timeout(HANDSHAKE_TIMEOUT, async {
        let (mut send, mut recv) = connection.accept_bi().await?;
        read_header(&mut recv, StreamId::Control).await?;
        let hello: ControlEnvelope = read_message(&mut recv).await?;
        let Some(control_envelope::Payload::ClientHello(hello)) = hello.payload else {
            bail!("Expected ClientHello on Control");
        };
        ensure!(
            hello.protocol_version == PROTOCOL_VERSION,
            "Unsupported protocol version: {}",
            hello.protocol_version
        );
        write_message(
            &mut send,
            &ControlEnvelope {
                payload: Some(control_envelope::Payload::ServerWelcome(
                    ControlServerWelcome {
                        protocol_version: PROTOCOL_VERSION,
                    },
                )),
            },
        )
        .await?;
        Ok::<_, anyhow::Error>((send, recv))
    })
    .await
    .context("Control handshake timed out")??;
    println!("Client joined: {}", connection.remote_address());

    tokio::try_join!(
        consume_control(control_send, control_recv),
        consume_events(connection),
    )?;
    Ok(())
}

async fn read_header(recv: &mut RecvStream, expected: StreamId) -> Result<()> {
    let header: StreamHeader = read_message(recv).await?;
    let channel = StreamId::try_from(header.stream_id).context("Unknown stream channel")?;
    ensure!(
        channel == expected,
        "Expected {expected:?} stream, received {channel:?}"
    );
    Ok(())
}

async fn consume_control(_send: SendStream, mut recv: RecvStream) -> Result<()> {
    let _: ControlEnvelope = read_message(&mut recv).await?;
    bail!("Unexpected Control message after Welcome")
}

async fn consume_events(connection: &Connection) -> Result<()> {
    let (_send, mut recv) = connection.accept_bi().await?;
    tokio::time::timeout(HANDSHAKE_TIMEOUT, read_header(&mut recv, StreamId::Events))
        .await
        .context("Events stream header timed out")??;
    println!("Events stream ready: {}", connection.remote_address());
    loop {
        let event: EventEnvelope = read_message(&mut recv).await?;
        ensure!(event.payload.is_some(), "Empty Events envelope");
    }
}
