---
title: Connection troubleshooting
sidebar:
  order: 4
---

## Certificate cannot be loaded

Start the server first, then run the client from the same working directory. The client reads `.sown/server-tls.pem` relative to its working directory.

If saved credentials are invalid, inspect the server's startup error and restore the correct credential file. The server preserves invalid saved credentials rather than silently replacing its identity.

## Connection times out

Check that the server prints `QUIC listening on 127.0.0.1:50420`. The reference client connects to that loopback address and applies a 5-second transport connection deadline.

If the address is already in use, stop the process occupying it before starting another server.

## Session is rejected

The application handshake has a 10-second deadline. A compatible client sends the Control stream header, sends Hello with version `1`, validates Welcome, and opens Events with its header.

Malformed framing, oversized messages, unsupported versions, invalid channel headers, duplicate Control streams, and unexpected Control messages cause rejection. Check the terminal reason against [Session establishment](../../wire/session/) and [Message framing](../../wire/framing/).

## Game window stays open after a failure

Connection failures appear in the terminal without closing the client window. Inspect the terminal for networking errors even when the scene remains visible.
