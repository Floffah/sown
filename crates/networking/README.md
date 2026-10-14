# Networking

Shared networking code uses `prost` for unsigned-varint message lengths and
protobuf encoding/decoding. The message body limit is 64 KiB.

- `decode_length` validates a buffered length prefix before body allocation.
- `decode_body` decodes a protobuf body without its prefix.
- `encode_message` creates a length-prefixed protobuf message.
- `ALPN` and `PROTOCOL_VERSION` define the connection's protocol identity.

Enable the `quic` feature to use `networking::quic::read_message` and
`write_message` with Quinn streams. The caller configures Quinn's runtime and
TLS features. Without `quic`, the crate provides buffered framing without a
transport dependency.
