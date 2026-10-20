---
title: Transports and channels
sidebar:
  order: 3
---

Native clients use QUIC, and browser clients use WebTransport over HTTP/3. Both carry Sown application messages, but establish connections differently.

Native QUIC negotiates ALPN `sown/1`. The application protocol version is the integer `1` in Hello and Welcome.

## Reliable channels

Control and Events are logical channels identified by `ClientStreamHeader`. Their identifiers are Protocol Buffers enum values, rather than QUIC-assigned stream IDs.

| Channel | Identifier | Purpose |
| --- | --- | --- |
| Control | `0` | Connection and session management |
| Events | `1` | Reliable world and gameplay messages |

Streams preserve order within a channel. Independent channels let Events processing continue while Control is idle. Reliable messages use [length-prefixed framing](../framing/).

## Datagrams

Datagrams carry transient state such as movement and snapshots without reliable retransmission. Receivers must tolerate loss and out-of-order delivery; retransmitting an obsolete update does not make it useful.

## Browser connection

A browser opens a WebTransport session through an HTTPS endpoint:

```ts
const transport = new WebTransport("https://luminous.floffah.dev/sown");
await transport.ready;
```

WebTransport uses HTTP/3 and QUIC; its connection setup differs from a native raw QUIC connection. Keep certificate validation enabled for either binding.
