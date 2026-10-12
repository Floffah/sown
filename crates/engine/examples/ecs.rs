use engine::ecs::{Ecs, bevy_ecs::prelude::*};

#[derive(Component)]
struct Counter(u32);

fn increment(mut counters: Query<&mut Counter>) {
    for mut counter in &mut counters {
        counter.0 += 1;
    }
}

fn main() {
    let mut ecs = Ecs::default();
    let entity = ecs.world.spawn(Counter(0)).id();
    ecs.schedule.add_systems(increment);

    for tick in 1..=3 {
        ecs.tick();
        let count = ecs.world.get::<Counter>(entity).unwrap().0;
        assert_eq!(count, tick);
        println!("Tick {tick}: counter = {count}");
    }
}
