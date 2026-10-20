---
title: Run a local server
sidebar:
  order: 2
---

From a Sown checkout with Rust and Just installed:

```sh
just run-server
```

The reference server loads the embedded Minimal Room world, validates its tree and transforms, and listens for native QUIC connections on `127.0.0.1:50420`. This is a loopback address, accessible from the same machine.

The server prints the loaded world name and listening address on startup, then logs connection status to the terminal. Start the [local client](/using/local-client/) from the same working directory so it can read the generated localhost certificate.

## World content

Both reference executables embed `fixtures/minimal-room/worlds/main.world`. Edit that file and recompile to change the scene. If you run an executable from another directory, it uses that directory for state and loads the same embedded world.

## Shutdown

Press Ctrl-C to close the server endpoint and wait for connections to shut down. Clients receive the connection closure.

## Session limits

The server permits two concurrent bidirectional streams per connection and no unidirectional streams. The Control handshake and Events header each have a 10-second deadline as described in [Session establishment](/wire/session/).
