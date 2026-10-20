---
title: Session establishment
sidebar:
  order: 5
---

A native session uses two client-opened, long-lived bidirectional streams on one QUIC connection.

## Control handshake

1. Open the first bidirectional stream.
2. Send a framed `ClientStreamHeader` with `stream_id = 0` (Control).
3. Send a framed `ControlEnvelope` containing `ControlClientHello` with `protocol_version = 1`.
4. Read the server's `ControlEnvelope` containing `ControlServerWelcome` on the same stream.
5. Check that Welcome reports protocol version `1`.

The server applies a 10-second deadline from accepting the connection through completing the Control handshake. The reference client applies a 10-second deadline around its application join operation.

## Events stream

After validating Welcome, open a second bidirectional stream and send a framed `ClientStreamHeader` with `stream_id = 1` (Events). The server applies a 10-second header deadline after accepting this stream.

Keep both stream pairs alive for the connection. The receiver handles Events independently of idle Control. The native client uses QUIC keepalives every 5 seconds without sending application messages.

## Rejection and closure

The reference handshake receiver rejects unexpected headers, unsupported versions, duplicate Control channels, malformed frames, empty Events envelopes, and any additional Control message after Welcome. The server closes rejected connections with application error code `1` and a textual reason.

A normal server shutdown closes the endpoint with code `0` and waits for connection shutdown. Transport admission and persistent identity authentication are separate operations.
