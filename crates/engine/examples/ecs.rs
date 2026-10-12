use engine::ecs::{
    Ecs,
    bevy_ecs::prelude::*,
    scene::{Camera, Material, Mesh, Transform, spawn_camera, spawn_cube},
};

fn inspect_scene(objects: Query<(Entity, &Transform, &Mesh, &Material)>) {
    for (entity, transform, mesh, material) in &objects {
        println!("{entity:?}: {transform:?}, {mesh:?}, {material:?}");
    }
}

fn inspect_camera(cameras: Query<(Entity, &Transform, &Camera)>) {
    for (entity, transform, camera) in &cameras {
        println!("{entity:?}: {transform:?}, {camera:?}");
    }
}

fn main() {
    let mut ecs = Ecs::default();
    spawn_cube(&mut ecs.world);
    spawn_camera(&mut ecs.world);
    ecs.schedule.add_systems((inspect_scene, inspect_camera));
    ecs.tick();
}
