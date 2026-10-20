use anyhow::{Result, bail, ensure};
use protocol::v1::{
    ClientStreamHeader, ControlClientHello, ControlEnvelope, StreamId, control_envelope,
};
use quinn::{Connection, RecvStream, SendStream};

use super::{ClientStreams, read_message, write_control, write_message};
use crate::PROTOCOL_VERSION;

/// Sends Hello on Control, validates Welcome, then opens Events.
/// The caller keeps the returned stream handles alive for the session.
///
/// # Errors
/// Returns an error for invalid responses, unsupported versions, or stream
/// failures. The caller applies a deadline and closes the connection on failure.
pub async fn handshake(connection: &Connection) -> Result<ClientStreams> {
    let (mut control_send, mut control_recv) = open_stream(connection, StreamId::Control).await?;
    let hello = ControlClientHello {
        protocol_version: PROTOCOL_VERSION,
        username: None,
    };
    write_control(
        &mut control_send,
        control_envelope::Payload::ClientHello(hello),
    )
    .await?;

    let response = read_message(&mut control_recv).await?;
    let peer_id = validate_welcome(response)?;

    let (events_send, events_recv) = open_stream(connection, StreamId::Events).await?;
    Ok(ClientStreams {
        peer_id,
        control_send,
        control_recv,
        events_send,
        events_recv,
    })
}

async fn open_stream(
    connection: &Connection,
    stream: StreamId,
) -> Result<(SendStream, RecvStream)> {
    let (mut send, recv) = connection.open_bi().await?;
    let header = ClientStreamHeader {
        stream_id: stream as i32,
    };
    write_message(&mut send, &header).await?;
    Ok((send, recv))
}

fn validate_welcome(response: ControlEnvelope) -> Result<u32> {
    match response.payload {
        Some(control_envelope::Payload::ServerWelcome(welcome)) => {
            ensure!(
                welcome.protocol_version == PROTOCOL_VERSION,
                "Unsupported server protocol version: {}",
                welcome.protocol_version
            );
            ensure!(welcome.peer_id != 0, "Invalid peer ID in Welcome");
            Ok(welcome.peer_id)
        }
        Some(control_envelope::Payload::ServerError(error)) => {
            bail!("Server rejected join: {}", error.message)
        }
        _ => bail!("Expected ServerWelcome on Control"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::v1::{ControlServerError, ControlServerWelcome};

    #[test]
    fn accepts_matching_welcome_and_rejects_other_responses() -> Result<()> {
        validate_welcome(ControlEnvelope {
            payload: Some(control_envelope::Payload::ServerWelcome(
                ControlServerWelcome {
                    protocol_version: PROTOCOL_VERSION,
                    peer_id: 1,
                },
            )),
        })?;
        for payload in [
            None,
            Some(control_envelope::Payload::ClientHello(ControlClientHello {
                protocol_version: PROTOCOL_VERSION,
                username: None,
            })),
            Some(control_envelope::Payload::ServerWelcome(
                ControlServerWelcome {
                    protocol_version: PROTOCOL_VERSION + 1,
                    peer_id: 1,
                },
            )),
        ] {
            assert!(validate_welcome(ControlEnvelope { payload }).is_err());
        }
        let error = validate_welcome(ControlEnvelope {
            payload: Some(control_envelope::Payload::ServerError(ControlServerError {
                message: "Server is full".into(),
            })),
        })
        .unwrap_err();
        assert!(error.to_string().contains("Server is full"));
        Ok(())
    }
}
