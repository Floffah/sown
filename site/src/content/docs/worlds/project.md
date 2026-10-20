---
title: Project layout
sidebar:
  order: 2
---

The Minimal Room project groups its manifest, worlds, scenes, and scripts as follows:

```text
game.json
worlds/
  main.world
scenes/
  marker.scene
src/
  shared/
    room.ts
  main/
    client/
      room.ts
    server/
      room.ts
```

## Manifest

`game.json` names the project and maps world names to source files:

```json
{
  "formatVersion": 1,
  "name": "Minimal Room",
  "worlds": {
    "main": {
      "source": "worlds/main.world",
      "default": true
    }
  }
}
```

`main` is the world's project key. `source` is its project-relative world file, and `default` identifies the default entry.

World files contain scene trees. The example `marker.scene` also contains a `formatVersion`, `name`, and `root` node. Scripts are grouped into shared, client, and server directories; [behaviour references](../behaviours/) show the corresponding source URIs.

Keep private server credentials outside source distributed to clients. Store generated cross-run server state under `.sown` in the process working directory.
