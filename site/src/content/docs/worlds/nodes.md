---
title: Nodes and geometry
sidebar:
  order: 4
---

Every node has an `id`, `name`, and `type`. Nodes form a tree through their `children` arrays.

| Field | Value |
| --- | --- |
| `id` | String identifying the authored node |
| `name` | Human-readable string |
| `type` | `Node3D` or `Mesh` |
| `transform` | Optional object with `position` and `scale` |
| `mesh` | Mesh information; required for `Mesh` nodes |
| `collider` | Optional object containing a `shape` string |
| `behaviours` | Array of script reference strings; defaults to empty |
| `scope` | Optional `replicated`, `server`, or `client` value |
| `children` | Array of child nodes; defaults to empty |

`Node3D` groups objects and supplies a parent transform. A `Mesh` node supplies geometry through `mesh.primitive`.

## Cube primitive

Use `"mesh": { "primitive": "cube" }` for a unit cube centred on the origin. Scale it to create floors, walls, and blocks. The source spelling is `cube`. Collider metadata uses its own shape string, such as `"collider": { "shape": "box" }`.

The reference loader assigns cube meshes an orange material with RGB colour `[0.9, 0.3, 0.15]`. Material colour is an engine component; it is not a world-file field.

## Metadata and simulation

The parser preserves behaviour references, scope, and collider metadata as source data. How that data is used depends on the code that loads it. The Rust scene converter creates node identity, hierarchy, transforms, and cube mesh components. A collider shape string alone does not apply collision response to player movement.
