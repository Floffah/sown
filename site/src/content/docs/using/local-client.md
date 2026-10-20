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
just run-client
```

The client opens the Minimal Room scene and connects to `127.0.0.1:50420`. It reads the server's localhost certificate from `.sown/server-tls.pem` in its working directory. Connection status and failures appear in the terminal. A connection failure leaves the game window open.

## Controls

| Key | Movement |
| --- | --- |
| W | Negative Z |
| S | Positive Z |
| A | Negative X |
| D | Positive X |
| Escape | Exit |

Movement follows world axes at 3 world units per second. Opposing keys cancel, and diagonal movement has the same maximum speed as movement on one axis. Losing focus, hiding, or suspending the window clears held input.

The camera starts at `(3, 3, 5)` looking toward the origin. When you move, its position changes.

For startup errors, see [Connection troubleshooting](../../hosting/troubleshooting/).
