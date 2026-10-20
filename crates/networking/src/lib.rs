//! Shared Sown connection constants and protobuf message framing.

use anyhow::{Context, Result, ensure};
use prost::Message;

#[cfg(feature = "quic")]
pub mod quic;
pub mod session;

pub const ALPN: &[u8] = b"sown/1";
pub const PROTOCOL_VERSION: u32 = 1;
pub const MAX_MESSAGE_SIZE: usize = 64 * 1024;

/// Decodes a protobuf length prefix and checks the message size limit.
///
/// # Errors
/// Returns an error for an invalid prefix or a length exceeding 64 KiB.
pub fn decode_length(prefix: &[u8]) -> Result<usize> {
    let length = prost::decode_length_delimiter(prefix).context("Invalid message length varint")?;
    ensure!(length <= MAX_MESSAGE_SIZE, "Message exceeds 64 KiB limit");
    Ok(length)
}

/// Decodes a protobuf message body without its length prefix.
///
/// # Errors
/// Returns an error for an oversized body or invalid protobuf data.
pub fn decode_body<M: Message + Default>(bytes: &[u8]) -> Result<M> {
    ensure!(
        bytes.len() <= MAX_MESSAGE_SIZE,
        "Message exceeds 64 KiB limit"
    );
    M::decode(bytes).context("Invalid protobuf message")
}

/// Encodes a protobuf message with an unsigned-varint length prefix.
///
/// # Errors
/// Returns an error if the message body exceeds 64 KiB.
pub fn encode_message<M: Message>(message: &M) -> Result<Vec<u8>> {
    ensure!(
        message.encoded_len() <= MAX_MESSAGE_SIZE,
        "Message exceeds 64 KiB limit"
    );
    Ok(message.encode_length_delimited_to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq, Message)]
    struct Payload {
        #[prost(bytes, tag = "1")]
        data: Vec<u8>,
    }

    #[test]
    fn framing_round_trips_and_rejects_invalid_lengths_and_bodies() -> Result<()> {
        for data in [vec![], vec![42; 200]] {
            let message = Payload { data };
            let encoded = encode_message(&message)?;
            let mut body = encoded.as_slice();
            let length = prost::decode_length_delimiter(&mut body)?;
            let prefix_length = encoded.len() - body.len();
            assert_eq!(decode_length(&encoded[..prefix_length])?, body.len());
            assert_eq!(length, body.len());
            assert_eq!(decode_body::<Payload>(body)?, message);
        }
        for prefix in [&[0x80][..], &[0x80; 10], &[0x81, 0x80, 0x04]] {
            assert!(decode_length(prefix).is_err());
        }
        let mut overflowing = [0x80; 10];
        overflowing[9] = 2;
        assert!(decode_length(&overflowing).is_err());
        assert_eq!(decode_length(&[0x80, 0x80, 0x04])?, MAX_MESSAGE_SIZE);
        assert!(decode_body::<Payload>(&[0xff]).is_err());
        let oversized = vec![0; MAX_MESSAGE_SIZE + 1];
        assert!(decode_body::<Payload>(&oversized).is_err());
        assert!(encode_message(&Payload { data: oversized }).is_err());
        Ok(())
    }
}
