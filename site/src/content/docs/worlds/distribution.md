---
title: Distributing world source
sidebar:
  order: 6
---

World servers advertise an accessible Git repository and a specific revision. Clients retrieve that revision to construct and inspect the client-side experience.

Use a Git host of your choice, including independently hosted services. Sown does not require a central marketplace or a particular commercial Git provider.

## Revisions

The advertised revision identifies the content associated with the world. Clients use that revision rather than substituting the repository's latest branch state. This keeps scene definitions, scripts, and assets tied to the same source version.

Your repository can contain world manifests, scene definitions, scripts, models, textures, audio, and other assets. Include the client content required to construct the experience, with clear licensing for code and assets.

## Trust and private data

Repository visibility does not make its contents safe to execute with host privileges. Clients construct worlds through their trusted, restricted runtime and treat downloaded content as untrusted.

Do not commit passwords, private keys, deployment credentials, or private player data to distributed source. An advertised repository also does not attest to the code running on the server.

See [Discovery](/wire/discovery/) for the metadata identifying a world's repository and revision.
