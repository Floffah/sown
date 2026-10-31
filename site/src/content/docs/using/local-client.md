---
title: Run the local client
sidebar:
  order: 5
---

From a Sown source checkout with Rust and Just installed, start the server:

```sh
just run-server
```

In another terminal, from the same checkout directory, start the client:

```sh
just run-client --connect 127.0.0.1:50420
```

The `--connect IP:PORT` development option connects to the specified server without starting a server inside the client. The client opens the Minimal Room scene and reads the server's localhost certificate from `.sown/server-tls.pem` in its working directory. Repeat the client command in another terminal to connect a second client to the same server. Both executables must use the same compiled world fixture.

Connection status and failures appear in the terminal. Missing or invalid certificates prevent startup. A transport or session failure leaves the game window open.

## Controls

| Key | Action |
| --- | --- |
| W | Negative Z |
| S | Positive Z |
| A | Negative X |
| D | Positive X |
| Escape | Exit |
| F12 | Toggle developer tools |

Movement follows world axes at 3 world units per second. Opposing keys cancel, and diagonal movement has the same maximum speed as movement on one axis. Losing focus, hiding, or suspending the window clears held input. Toggling developer tools also clears held movement keys; press them again to resume movement.

The camera starts at `(3, 3, 5)` looking toward the origin. When you move, its position changes.

For startup errors, see [Connection troubleshooting](../../hosting/troubleshooting/).
