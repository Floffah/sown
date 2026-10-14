//! Standalone ECS state and scheduling, independent of the scene and renderer.

pub use bevy_ecs;
use bevy_ecs::prelude::{Resource, Schedule, World};
use std::time::Duration;

pub mod player;
pub mod scene;
mod source;

/// An ECS world with a schedule that the caller advances explicitly.
pub struct Ecs {
    pub world: World,
    pub schedule: Schedule,
}

impl Default for Ecs {
    fn default() -> Self {
        let mut world = World::new();
        world.insert_resource(player::MovementIntent::default());
        world.insert_resource(FrameTime::default());
        let mut schedule = Schedule::default();
        schedule.add_systems(player::move_local_player);
        Self { world, schedule }
    }
}

impl Ecs {
    /// Advances the registered systems by the elapsed time supplied by the caller.
    pub fn tick(&mut self, elapsed: Duration) {
        self.world.resource_mut::<FrameTime>().0 = elapsed;
        self.schedule.run(&mut self.world);
    }
}

#[derive(Resource, Default)]
struct FrameTime(Duration);

#[cfg(test)]
mod tests {
    use super::{Ecs, bevy_ecs::prelude::*};

    #[derive(Component)]
    struct Counter(u32);

    fn increment(mut counters: Query<&mut Counter>) {
        for mut counter in &mut counters {
            counter.0 += 1;
        }
    }

    #[test]
    fn ticks_run_systems_and_preserve_state() {
        let mut ecs = Ecs::default();
        let entity = ecs.world.spawn(Counter(0)).id();
        ecs.schedule.add_systems(increment);

        for expected in 1..=3 {
            ecs.tick(std::time::Duration::ZERO);
            assert_eq!(ecs.world.get::<Counter>(entity).unwrap().0, expected);
        }
    }
}
