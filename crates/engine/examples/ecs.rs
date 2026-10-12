use engine::ecs::{
    Ecs,
    bevy_ecs::prelude::*,
    scene::{Material, Mesh, Transform, spawn_cube},
};

fn inspect_scene(objects: Query<(Entity, &Transform, &Mesh, &Material)>) {
    for (entity, transform, mesh, material) in &objects {
        println!("{entity:?}: {transform:?}, {mesh:?}, {material:?}");
    }
}

fn main() {
    let mut ecs = Ecs::default();
    spawn_cube(&mut ecs.world);
    ecs.schedule.add_systems(inspect_scene);
    ecs.tick();
}
