# Server

Run `just run-server` from the workspace root.

The server embeds `fixtures/minimal-room/worlds/main.world`, parses and validates
it through `project`, and retains the resulting engine ECS for its lifetime.
Quinn listens on `127.0.0.1:50420` with ALPN `sown/1`. Each accepted connection
has an asynchronous task that logs its connection and disconnection.

Clients open two long-lived bidirectional streams on one connection. The first
is Control: send a `StreamHeader` identifying Control, then a `ControlEnvelope`
containing `ClientHello` with protocol version `1`. The server replies on that
same stream with a `ControlEnvelope` containing `ServerWelcome`, version `1`.
The handshake must complete within 10 seconds of the QUIC connection being
accepted. After Welcome, clients open Events and send its `StreamHeader` once;
the server reads and validates `EventEnvelope` messages on it independently of
Control. The Events header must arrive within 10 seconds of accepting its stream.

Every header and envelope is encoded as an unsigned-varint byte length followed
by the protobuf message, with a 64 KiB body limit. Shared framing and protocol
identity constants live in `crates/networking`; the server uses its `quic`
feature. Channel identifiers are the
protobuf `StreamId` values, not QUIC stream IDs. Invalid headers, envelopes,
unsupported versions, duplicate Control streams, and handshake timeouts close
the connection with application error code `1` and a reason. An additional
Control message after Welcome is rejected. The endpoint permits two concurrent
bidirectional streams and no unidirectional streams.

TLS uses a self-signed `localhost` certificate and private key stored together
in `.sown/server-tls.pem`, relative to the process working directory. The server
generates this file on first startup and reuses it on subsequent runs. Invalid
credentials cause startup to fail without replacing the saved identity. The
file is created with owner-only permissions on Unix and `.sown` is ignored by
Git. The loopback handshake test explicitly trusts its generated certificate.

Ctrl-C closes the endpoint and waits for connections to shut down.
