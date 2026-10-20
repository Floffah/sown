---
title: World discovery
sidebar:
  order: 2
---

A world address such as `sown://luminous.floffah.dev` identifies a host. Public discovery information is served over HTTPS without authentication:

```http
GET /.well-known/sown
```

The discovery response identifies the protocol, world name, Git repository revision, and session transport endpoints:

```json
{
  "protocol": "sown/1",
  "world": {
    "name": "Luminous",
    "repository": "https://github.com/floffah/luminous",
    "revision": "a1b2c3d4"
  },
  "session": {
    "host": "luminous.floffah.dev",
    "port": 50420,
    "quicSupported": true,
    "wtSupported": true
  }
}
```

The native session uses the advertised QUIC endpoint. Browser sessions use the HTTPS WebTransport endpoint at `/sown`.

## Source selection and trust

Retrieve the advertised repository revision. Substituting the latest branch content can combine mismatched world definitions, scripts, and assets.

Validate HTTPS peer identity and treat discovery data as untrusted input. HTTPS protects transport integrity; it does not establish that the repository is safe or that the server executes the advertised source. The client enforces world isolation independently.
