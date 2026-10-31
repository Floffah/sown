use anyhow::{Result, bail, ensure};
use protocol::v1::{
    ClientStreamHeader, ControlClientEnvelope, ControlClientHello, ControlServerEnvelope, StreamId,
    control_client_envelope, control_server_envelope,
};
use quinn::{Connection, RecvStream, SendStream};

use super::{ClientStreams, read_message, write_message};
use crate::{
    PROTOCOL_VERSION,
    inspection::{Capture, CaptureContext, Channel},
};

/// Sends Hello on Control, validates Welcome, then opens Events.
/// The caller keeps the returned stream handles alive for the session.
///
/// # Errors
/// Returns an error for invalid responses, unsupported versions, or stream
/// failures. The caller applies a deadline and closes the connection on failure.
pub async fn handshake(
    connection: &Connection,
    capture: Option<&Capture>,
) -> Result<ClientStreams> {
    let connection_id = connection.stable_id() as u64;
    let control_capture = capture.map(|capture| capture.context(connection_id, Channel::Control));
    let events_capture = capture.map(|capture| capture.context(connection_id, Channel::Events));
    let (mut control_send, mut control_recv) =
        open_stream(connection, StreamId::Control, control_capture.as_ref()).await?;
    write_message(
        &mut control_send,
        &ControlClientEnvelope {
            payload: Some(control_client_envelope::Payload::ClientHello(
                ControlClientHello {
                    protocol_version: PROTOCOL_VERSION,
                    username: None,
                },
            )),
        },
        control_capture.as_ref(),
    )
    .await?;

    let response = read_message(&mut control_recv, control_capture.as_ref()).await?;
    let peer_id = validate_welcome(response)?;

    let (events_send, events_recv) =
        open_stream(connection, StreamId::Events, events_capture.as_ref()).await?;
    Ok(ClientStreams {
        peer_id,
        control_send,
        control_recv,
        events_send,
        events_recv,
        control_capture,
        events_capture,
    })
}

async fn open_stream(
    connection: &Connection,
    stream: StreamId,
    capture: Option<&CaptureContext>,
) -> Result<(SendStream, RecvStream)> {
    let (mut send, recv) = connection.open_bi().await?;
    write_message(
        &mut send,
        &ClientStreamHeader {
            stream_id: stream as i32,
        },
        capture,
    )
    .await?;
    Ok((send, recv))
}

fn validate_welcome(response: ControlServerEnvelope) -> Result<u32> {
    match response.payload {
        Some(control_server_envelope::Payload::ServerWelcome(welcome)) => {
            ensure!(
                welcome.protocol_version == PROTOCOL_VERSION,
                "Unsupported server protocol version: {}",
                welcome.protocol_version
            );
            ensure!(welcome.peer_id != 0, "Invalid peer ID in Welcome");
            Ok(welcome.peer_id)
        }
        Some(control_server_envelope::Payload::ServerError(error)) => {
            bail!("Server rejected join: {}", error.message)
        }
        _ => bail!("Expected ServerWelcome on Control"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::v1::{ControlServerError, ControlServerJoined, ControlServerWelcome};

    #[test]
    fn accepts_matching_welcome_and_rejects_other_responses() -> Result<()> {
        validate_welcome(ControlServerEnvelope {
            payload: Some(control_server_envelope::Payload::ServerWelcome(
                ControlServerWelcome {
                    protocol_version: PROTOCOL_VERSION,
                    peer_id: 1,
                },
            )),
        })?;
        for payload in [
            None,
            Some(control_server_envelope::Payload::ServerJoined(
                ControlServerJoined {
                    player_entity_id: 1,
                },
            )),
            Some(control_server_envelope::Payload::ServerWelcome(
                ControlServerWelcome {
                    protocol_version: PROTOCOL_VERSION + 1,
                    peer_id: 1,
                },
            )),
        ] {
            assert!(validate_welcome(ControlServerEnvelope { payload }).is_err());
        }
        let error = validate_welcome(ControlServerEnvelope {
            payload: Some(control_server_envelope::Payload::ServerError(
                ControlServerError {
                    message: "Server is full".into(),
                },
            )),
        })
        .unwrap_err();
        assert!(error.to_string().contains("Server is full"));
        Ok(())
    }
}
