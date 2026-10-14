//! Length-prefixed protobuf messages over Quinn streams.

use anyhow::{Context, Result, bail};
use prost::Message;
use quinn::{RecvStream, SendStream};

use crate::{decode_body, decode_length, encode_message};

/// Reads one length-prefixed protobuf message from a QUIC stream.
///
/// Cancelling this operation can consume part of a frame. Close the stream
/// rather than restarting a cancelled read on the same stream.
///
/// # Errors
/// Returns an error for truncated frames, invalid protobuf data, oversized
/// messages, or a stream read failure.
pub async fn read_message<M: Message + Default>(recv: &mut RecvStream) -> Result<M> {
    let mut prefix = [0; 10];
    for index in 0..prefix.len() {
        recv.read_exact(&mut prefix[index..=index])
            .await
            .context("Truncated message length")?;
        if prefix[index] & 0x80 == 0 {
            let length = decode_length(&prefix[..=index])?;
            let mut bytes = vec![0; length];
            recv.read_exact(&mut bytes)
                .await
                .context("Truncated message body")?;
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
