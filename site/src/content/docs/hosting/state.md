---
title: Certificates and persistent state
sidebar:
  order: 3
---

Sown stores managed cross-run state under `.sown` in the process working directory. User-editable configuration belongs directly in that working directory. Neither location is derived from the executable's installation directory.

## Local TLS identity

The reference server stores its self-signed `localhost` certificate and private key together in:

```text
.sown/server-tls.pem
```

The server generates this file on first startup and reuses it on each restart. Invalid saved credentials cause startup to fail without replacing the file. On Unix, the server creates the credential file with owner-only permissions.

The native client reads the certificate from the same relative path and trusts it for the localhost connection. Start client and server from the same directory so they use the same certificate. TLS authenticates the server connection; it does not establish a player's identity.

## Handling state

Keep `.sown` out of Git and protect the credential file because it contains a private key. Keep the file to preserve the server's identity across restarts. If you remove it, the server generates a new identity on startup.

`just run-server` configures `SSLKEYLOGFILE` as `.sown/tls-keys.log`. Debug builds write TLS keys to this log. Anyone with these keys can decrypt the packets, so protect the log like other secret diagnostic material.
