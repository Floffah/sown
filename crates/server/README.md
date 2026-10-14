# Server

Run `just run-server` from the workspace root.

The server embeds `fixtures/minimal-room/worlds/main.world`, parses and validates
it through `project`, and retains the resulting engine ECS for its lifetime.
Quinn listens on `127.0.0.1:50420` with ALPN `sown/1`. Each accepted connection
has an asynchronous task that logs its connection and disconnection.

TLS uses a self-signed `localhost` certificate and private key stored together
in `.sown/server-tls.pem`, relative to the process working directory. The server
generates this file on first startup and reuses it on subsequent runs. Invalid
credentials cause startup to fail without replacing the saved identity. The
file is created with owner-only permissions on Unix and `.sown` is ignored by
Git. The loopback handshake test explicitly trusts its generated certificate.

Ctrl-C closes the endpoint and waits for connections to shut down.
