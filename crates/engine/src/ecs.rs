//! Standalone ECS state and scheduling, independent of the scene and renderer.

pub mod scene;

pub use bevy_ecs;
use bevy_ecs::prelude::{Schedule, World};

/// An ECS world with a schedule that the caller advances explicitly.
#[derive(Default)]
pub struct Ecs {
    pub world: World,
    pub schedule: Schedule,
}

impl Ecs {
    /// Runs the registered systems once against this instance's world.
    pub fn tick(&mut self) {
        self.schedule.run(&mut self.world);
    }
}

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
            ecs.tick();
            assert_eq!(ecs.world.get::<Counter>(entity).unwrap().0, expected);
        }
    }
}
