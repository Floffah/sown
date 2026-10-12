# Engine ECS

The engine owns a standalone Bevy ECS layer in `engine::ecs::Ecs`.
Create an instance with `Ecs::default()`, spawn entities and insert resources
through `ecs.world`, register systems through `ecs.schedule.add_systems(...)`,
and call `ecs.tick()` to run the schedule once. State persists between ticks.
Bevy's types and derives are available through `engine::ecs::bevy_ecs`.

Run the headless example with:

```sh
cargo run --locked -p engine --example ecs
```

The example builds a single-cube scene directly in ECS and runs a system that
queries and prints its components. `engine::ecs::scene::spawn_cube` creates one
entity with:

- `Transform`: translation at the origin, identity quaternion rotation (x, y, z,
  w), and unit scale.
- `Mesh::Cube`: unit-cube geometry, centred on the origin.
- `Material`: an orange RGB colour.

These are ordinary ECS components. No mesh vertices, shader, or GPU resources
are created yet. The cube scene is authored in Rust; it has no parser or parent
hierarchy.

To inspect scene objects, query their components. In Bevy ECS 0.19,
`world.entities().len()` counts allocated entity metadata slots, including unused
slots; it does not count scene objects. Resources also occupy entities, including
the `DefaultQueryFilters` resource created by `World::new()`.

The ECS does not yet run in the client loop or interact with the renderer or
prototype scene. Only Bevy ECS's `std` feature is enabled; this first step uses
single-threaded scheduling.
