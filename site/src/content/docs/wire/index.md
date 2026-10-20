---
title: Wire protocol
sidebar:
  order: 1
---

The Sown wire protocol connects clients to independently operated world servers. World source distribution, live sessions, and player identity are separate responsibilities.

Read these pages in connection order:

1. [Discovery](/wire/discovery/): find the repository revision and transport endpoints.
2. [Transport](/wire/transport/): native QUIC and browser WebTransport.
3. [Framing](/wire/framing/): length-prefixed Protocol Buffers on reliable streams.
4. [Session establishment](/wire/session/): channel headers and Hello/Welcome.
5. [Messages and snapshots](/wire/messages/): peer, entity, and movement data.

[Networking helpers](/wire/rust/) explains the reference Rust API. [Identity](/identity/) describes persistent players independently of their transport connection.
