---
title: Load and simulate worlds
sidebar:
  order: 2
---

Parse JSON world source and convert it to live scene state:

```rust
let source = project::world::parse(world_json)?;
let mut ecs = engine::ecs::Ecs::from_source(&source)?;
```

`parse` returns a `WorldSource` or `serde_json::Error`. `Ecs::from_source` creates one entity per node, preserving ID, name, node type, and hierarchy. A conversion error returns no partial ECS.

## Scene state

`Node` stores the identity from the world file. `LocalTransform` stores the parent-relative transform, and `Transform` stores the world-space transform the renderer uses. `ChildOf` and `Children` preserve parent-child relationships.

The loader composes world transforms. If you edit transforms through the ECS, update the corresponding world transforms too; changing `LocalTransform` does not automatically update descendants.

## Systems and time

`Ecs::default()` creates an empty ECS with the local-player movement system. Use `ecs.world` to spawn entities and insert resources, `ecs.schedule.add_systems(...)` to register systems, and `ecs.tick(elapsed)` to advance the schedule once. `elapsed` is a `std::time::Duration`. State persists between ticks. Bevy types and derives are re-exported through `engine::ecs::bevy_ecs`.

## Player input

`engine::ecs::player::spawn_local_player(&mut ecs.world)` creates a player with a camera. Supply `MovementIntent { x, z }` as a resource before ticking. Movement applies to entities with both `Player` and `LocalPlayer` markers.

Each axis is clamped to `[-1, 1]`, combined magnitude is limited to one, and smaller analog values preserve their strength. Non-finite input produces no movement. Movement changes X and Z at up to 3 world units per second while preserving Y, rotation, and scale.
