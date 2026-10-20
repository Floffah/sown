---
title: Behaviour references
sidebar:
  order: 5
---

A node's `behaviours` array contains script references. The Minimal Room root attaches one server file and one client file:

```json
"behaviours": [
  "root://main/server/room.ts",
  "root://main/client/room.ts"
]
```

The example files live under `src/main/server` and `src/main/client`. Shared source lives under `src/shared` and is referenced as `root://shared/room.ts`:

```ts
import { roomName } from "root://shared/room.ts";

export const label = `${roomName}:client`;
```

The shared file exports the project label:

```ts
export const roomName = "Minimal Room";
```

These references attach source files to nodes. The JSON parser stores them as strings; parsing a world does not execute attached scripts. Scripts run inside the world's restricted runtime.

Keep secrets out of any source made available to visitors. A server directory name is not an access-control boundary for a publicly accessible repository.
