---
title: World file format
sidebar:
  order: 3
---

A world file is a JSON object with `formatVersion`, `name`, and one `root` node:

```json
{
  "formatVersion": 1,
  "name": "My World",
  "root": {
    "id": "world-root",
    "name": "World",
    "type": "Node3D",
    "children": [
      {
        "id": "floor",
        "name": "Floor",
        "type": "Mesh",
        "transform": {
          "position": [0, -0.5, 0],
          "scale": [20, 1, 20]
        },
        "mesh": { "primitive": "cube" }
      }
    ]
  }
}
```

| Field | Value |
| --- | --- |
| `formatVersion` | Unsigned 32-bit integer identifying the format version |
| `name` | World name as a string |
| `root` | A [node object](/worlds/nodes/) containing the hierarchy |

The parser checks required fields, field types, unknown fields, node types, mesh primitives, and mesh requirements throughout the tree. Unknown fields cause an error. Choose distinct node IDs for references within your world.

## Coordinates

Positions and scales contain three JSON numbers in X, Y, Z order. Positive Y is up. A transform contains `position` and `scale`; either field can be omitted. The loader uses zero position, unit scale, and identity rotation for omitted values.

Transforms are relative to the parent. Parent scale affects child position and scale; parent translation offsets child position. Loading rejects non-finite or overflowing world-space transforms.
