---
title: Reliable message framing
sidebar:
  order: 4
---

A reliable stream is a sequence of frames. Each frame contains an unsigned-varint byte length followed by that many bytes of Protocol Buffers data:

```text
unsigned varint length | protobuf message body
```

The length describes only the body, excluding the prefix. Each header and each envelope has its own frame. Send a stream header once, followed by channel-specific envelopes.

## Limits and decoding

The maximum body length is **65,536 bytes (64 KiB)**. Validate the prefix before allocating storage for the body. Reject invalid or overflowing varints, oversized lengths, truncated bodies, and malformed Protocol Buffers messages.

The body type depends on where you are in the stream: `ClientStreamHeader` identifies the channel, then `ControlEnvelope` or `EventEnvelope` supplies messages. A stream read can contain part of a frame or several frames, so read boundaries do not identify message boundaries.

## Compatibility

Preserve Protocol Buffers field numbers and reserved fields when implementing the schema. An envelope uses a `oneof` payload to identify its message. An empty Events envelope is rejected by the reference session receiver.

Datagrams have transport-provided boundaries. The reliable stream prefix is not an extra requirement on the datagram schema.
