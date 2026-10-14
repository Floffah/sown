# Client

Start `just run-server`, then `just run-client` from the same workspace root.
The native client opens its game window and attempts one QUIC connection to
`127.0.0.1:50420` on a Tokio worker thread. It trusts the localhost certificate
read from `.sown/server-tls.pem` in its working directory. Connection status and
failures are printed to the terminal; connection failures leave the window running.

The server closes connections that do not complete its application handshake
within 10 seconds. This is the expected disconnect reason for this client.
