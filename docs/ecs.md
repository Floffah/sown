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

The example increments a component over three ticks and checks each result.
The ECS does not yet run in the client loop or interact with the renderer or
prototype scene. Only Bevy ECS's `std` feature is enabled; this first step uses
single-threaded scheduling.
