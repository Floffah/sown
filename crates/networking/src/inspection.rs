//! Non-blocking capture of complete Sown message bodies.

use std::{
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
        mpsc::{self, Receiver, SyncSender},
    },
    time::Duration,
};

use anyhow::Result;
use prost::Message;
use serde::Serialize;
use serde_json::Value;
use web_time::Instant;

const QUEUE_CAPACITY: usize = 256;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Received,
    Sent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    Control,
    Events,
    Datagram,
}

/// A complete plaintext message, captured independently of protocol validation.
pub struct Packet {
    pub elapsed: Duration,
    pub connection_id: u64,
    pub channel: Channel,
    pub direction: Direction,
    pub message_type: &'static str,
    pub wire_size: usize,
    pub body: Vec<u8>,
    decode_json: fn(&[u8]) -> Result<Value>,
}

impl Packet {
    /// Decodes the captured body using its protobuf type and generated JSON mapping.
    ///
    /// # Errors
    /// Returns an error if the body cannot be decoded or represented as JSON.
    pub fn json(&self) -> Result<Value> {
        (self.decode_json)(&self.body)
    }
}

#[derive(Clone)]
pub struct Capture {
    sender: SyncSender<Packet>,
    dropped: Arc<AtomicU64>,
    started: Instant,
}

impl Capture {
    #[must_use]
    pub fn context(&self, connection_id: u64, channel: Channel) -> CaptureContext {
        CaptureContext {
            capture: self.clone(),
            connection_id,
            channel,
        }
    }
}

/// Connection and channel metadata supplied at a message boundary.
pub struct CaptureContext {
    capture: Capture,
    connection_id: u64,
    channel: Channel,
}

impl CaptureContext {
    /// Copies a complete typed message body into the capture queue without blocking.
    /// Oversized bodies and full queues increment the dropped-record counter.
    pub fn record<M: Message + Default + Serialize>(
        &self,
        direction: Direction,
        body: &[u8],
        wire_size: usize,
    ) {
        if body.len() > crate::MAX_MESSAGE_SIZE {
            self.capture.dropped.fetch_add(1, Ordering::Relaxed);
            return;
        }
        let type_name = std::any::type_name::<M>();
        let packet = Packet {
            elapsed: self.capture.started.elapsed(),
            connection_id: self.connection_id,
            channel: self.channel,
            direction,
            message_type: type_name.rsplit("::").next().unwrap_or(type_name),
            wire_size,
            body: body.to_vec(),
            decode_json: decode_json::<M>,
        };
        if self.capture.sender.try_send(packet).is_err() {
            self.capture.dropped.fetch_add(1, Ordering::Relaxed);
        }
    }
}

fn decode_json<M: Message + Default + Serialize>(body: &[u8]) -> Result<Value> {
    Ok(serde_json::to_value(M::decode(body)?)?)
}

pub struct CaptureReceiver {
    receiver: Receiver<Packet>,
    dropped: Arc<AtomicU64>,
}

impl CaptureReceiver {
    #[must_use]
    pub fn try_recv(&self) -> Option<Packet> {
        self.receiver.try_recv().ok()
    }

    #[must_use]
    pub fn dropped(&self) -> u64 {
        self.dropped.load(Ordering::Relaxed)
    }
}

/// Creates a bounded capture queue. Full queues drop observations rather than block.
#[must_use]
pub fn capture_channel() -> (Capture, CaptureReceiver) {
    let (sender, receiver) = mpsc::sync_channel(QUEUE_CAPACITY);
    let dropped = Arc::new(AtomicU64::new(0));
    (
        Capture {
            sender,
            dropped: dropped.clone(),
            started: Instant::now(),
        },
        CaptureReceiver { receiver, dropped },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::v1::ControlClientHello;

    #[test]
    fn full_capture_queue_drops_records_and_preserves_original_bytes() -> Result<()> {
        let (capture, receiver) = capture_channel();
        let context = capture.context(12, Channel::Control);
        let body = ControlClientHello {
            protocol_version: 1,
            username: Some("Ada".into()),
        }
        .encode_to_vec();
        for _ in 0..QUEUE_CAPACITY + 5 {
            context.record::<ControlClientHello>(Direction::Sent, &body, body.len() + 1);
        }
        assert_eq!(receiver.dropped(), 5);
        let packet = receiver.try_recv().unwrap();
        assert_eq!(packet.body, body);
        assert_eq!(packet.direction, Direction::Sent);
        assert_eq!(packet.connection_id, 12);
        assert_eq!(packet.channel, Channel::Control);
        assert_eq!(
            packet.json()?,
            serde_json::json!({"protocolVersion": 1, "username": "Ada"})
        );
        Ok(())
    }

    #[test]
    fn malformed_message_is_visible_to_the_capture_consumer() {
        let (capture, receiver) = capture_channel();
        let context = capture.context(12, Channel::Events);
        context.record::<ControlClientHello>(Direction::Received, &[0xff], 2);
        let packet = receiver.try_recv().unwrap();
        assert_eq!(packet.direction, Direction::Received);
        assert_eq!(packet.body, [0xff]);
        assert!(packet.json().is_err());
    }
}
