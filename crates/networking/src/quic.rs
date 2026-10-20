//! QUIC stream handles and length-prefixed protobuf messages.

use anyhow::{Context, Result, bail};
use prost::Message;
use protocol::v1::{ControlEnvelope, control_envelope};
use quinn::{RecvStream, SendStream};
use serde::Serialize;

use crate::inspection::{CaptureContext, Direction};
use crate::{decode_body, decode_length, encode_message};

mod handshake;
mod join;

pub use handshake::handshake;
pub use join::join;

/// Open Control and Events streams and the peer ID assigned during Handshake.
pub struct ClientStreams {
    pub peer_id: u32,
    pub control_send: SendStream,
    pub control_recv: RecvStream,
    pub events_send: SendStream,
    pub events_recv: RecvStream,
    pub control_capture: Option<CaptureContext>,
    pub events_capture: Option<CaptureContext>,
}

/// Reads one length-prefixed protobuf message from a QUIC stream.
///
/// Cancelling this operation can consume part of a frame. Close the stream
/// instead.
///
/// # Errors
/// Returns an error for truncated frames, invalid protobuf data, oversized
/// messages, or a stream read failure.
pub async fn read_message<M: Message + Default + Serialize>(
    recv: &mut RecvStream,
    capture: Option<&CaptureContext>,
) -> Result<M> {
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
            if let Some(capture) = capture {
                capture.record::<M>(Direction::Received, &bytes, index + 1 + length);
            }
            return decode_body(&bytes);
        }
    }
    bail!("Invalid message length varint")
}

/// Wraps a Control payload in its envelope and writes the framed message.
///
/// # Errors
/// Returns an error if the message is oversized or the stream write fails.
pub async fn write_control(
    send: &mut SendStream,
    payload: control_envelope::Payload,
    capture: Option<&CaptureContext>,
) -> Result<()> {
    let envelope = ControlEnvelope {
        payload: Some(payload),
    };
    write_message(send, &envelope, capture).await
}

/// Writes one protobuf message with its length prefix to a QUIC stream.
///
/// # Errors
/// Returns an error if the message is oversized or the stream write fails.
pub async fn write_message<M: Message + Default + Serialize>(
    send: &mut SendStream,
    message: &M,
    capture: Option<&CaptureContext>,
) -> Result<()> {
    let bytes = encode_message(message)?;
    send.write_all(&bytes).await?;
    if let Some(capture) = capture {
        let body = &bytes[bytes.len() - message.encoded_len()..];
        capture.record::<M>(Direction::Sent, body, bytes.len());
    }
    Ok(())
}
