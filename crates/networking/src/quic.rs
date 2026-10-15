//! Length-prefixed protobuf messages over Quinn streams.

use anyhow::{Context, Result, bail, ensure};
use prost::Message;
use protocol::v1::{ControlClientHello, ControlEnvelope, StreamHeader, StreamId, control_envelope};
use quinn::{Connection, RecvStream, SendStream};

use crate::{PROTOCOL_VERSION, decode_body, decode_length, encode_message};

/// Open Control and Events streams retained for the joined session.
pub struct ClientStreams {
    pub control_send: SendStream,
    pub control_recv: RecvStream,
    pub events_send: SendStream,
    pub events_recv: RecvStream,
}

/// Sends Hello on Control, validates Welcome, then opens Events.
/// The caller keeps the returned stream handles alive for the session.
///
/// # Errors
/// Returns an error for invalid responses, unsupported versions, or stream
/// failures. The caller applies a deadline and closes the connection on failure.
pub async fn join(connection: &Connection) -> Result<ClientStreams> {
    let (mut control_send, mut control_recv) = connection.open_bi().await?;
    write_message(
        &mut control_send,
        &StreamHeader {
            stream_id: StreamId::Control as i32,
        },
    )
    .await?;
    write_message(
        &mut control_send,
        &ControlEnvelope {
            payload: Some(control_envelope::Payload::ClientHello(ControlClientHello {
                protocol_version: PROTOCOL_VERSION,
                username: None,
            })),
        },
    )
    .await?;
    let response: ControlEnvelope = read_message(&mut control_recv).await?;
    validate_welcome(response)?;
    let (mut events_send, events_recv) = connection.open_bi().await?;
    write_message(
        &mut events_send,
        &StreamHeader {
            stream_id: StreamId::Events as i32,
        },
    )
    .await?;
    Ok(ClientStreams {
        control_send,
        control_recv,
        events_send,
        events_recv,
    })
}

fn validate_welcome(response: ControlEnvelope) -> Result<()> {
    match response.payload {
        Some(control_envelope::Payload::ServerWelcome(welcome)) => {
            ensure!(
                welcome.protocol_version == PROTOCOL_VERSION,
                "Unsupported server protocol version: {}",
                welcome.protocol_version
            );
        }
        Some(control_envelope::Payload::ServerError(error)) => {
            bail!("Server rejected join: {}", error.message)
        }
        _ => bail!("Expected ServerWelcome on Control"),
    }
    Ok(())
}

/// Reads one length-prefixed protobuf message from a QUIC stream.
///
/// Cancelling this operation can consume part of a frame. Close the stream
/// instead.
///
/// # Errors
/// Returns an error for truncated frames, invalid protobuf data, oversized
/// messages, or a stream read failure.
pub async fn read_message<M: Message + Default>(recv: &mut RecvStream) -> Result<M> {
    let mut prefix = [0; 10];
    for index in 0..prefix.len() {
        recv.read_exact(&mut prefix[index..=index])
            .await
            .context("Failed to read message length")?;
        if prefix[index] & 0x80 == 0 {
            let length = decode_length(&prefix[..=index])?;
            let mut bytes = vec![0; length];
            recv.read_exact(&mut bytes)
                .await
                .context("Failed to read message body")?;
            return decode_body(&bytes);
        }
    }
    bail!("Invalid message length varint")
}

/// Writes one protobuf message with its length prefix to a QUIC stream.
///
/// # Errors
/// Returns an error if the message is oversized or the stream write fails.
pub async fn write_message<M: Message>(send: &mut SendStream, message: &M) -> Result<()> {
    send.write_all(&encode_message(message)?).await?;
    Ok(())
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
