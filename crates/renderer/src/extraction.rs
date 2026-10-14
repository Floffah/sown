use std::collections::HashMap;

use engine::ecs::{
    bevy_ecs::prelude::{With, World},
    player::{LocalPlayer, Player},
    scene::{Camera, Material, Mesh, Transform},
};

pub(super) fn extract_camera(world: &mut World, aspect: f32) -> anyhow::Result<[f32; 12]> {
    let mut cameras =
        world.query_filtered::<(&Camera, &Transform), (With<Player>, With<LocalPlayer>)>();
    let (camera, transform) = cameras.single(world).map_err(|error| {
        anyhow::anyhow!("Expected exactly one local player with Camera and Transform: {error}")
    })?;
    anyhow::ensure!(
        camera.fov_y_radians.is_finite()
            && camera.fov_y_radians > 0.0
            && camera.fov_y_radians < std::f32::consts::PI
            && camera.near.is_finite()
            && camera.near > 0.0
            && camera.far.is_finite()
            && camera.far > camera.near,
        "Camera requires a field of view between 0 and pi radians and 0 < near < far"
    );
    let length = transform
        .rotation
        .iter()
        .map(|value| value * value)
        .sum::<f32>()
        .sqrt();
    anyhow::ensure!(
        transform.translation.iter().all(|value| value.is_finite())
            && length.is_finite()
            && length > 0.0,
        "Camera requires a finite position and a finite, nonzero quaternion"
    );
    let [x, y, z] = transform.translation;
    let [qx, qy, qz, qw] = transform.rotation.map(|value| value / length);
    Ok([
        x,
        y,
        z,
        0.0,
        qx,
        qy,
        qz,
        qw,
        1.0 / (camera.fov_y_radians / 2.0).tan(),
        aspect,
        camera.near,
        camera.far,
    ])
}

// Each instance contains translation (3), rotation (4), scale (3), and colour (3).
pub(super) fn extract_meshes(world: &mut World) -> HashMap<Mesh, Vec<u8>> {
    let mut meshes = HashMap::<Mesh, Vec<u8>>::new();
    for (transform, mesh, material) in world.query::<(&Transform, &Mesh, &Material)>().iter(world) {
        meshes.entry(*mesh).or_default().extend(
            transform
                .translation
                .into_iter()
                .chain(transform.rotation)
                .chain(transform.scale)
                .chain(material.color)
                .flat_map(f32::to_le_bytes),
        );
    }
    meshes
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use engine::ecs::scene::MeshPrimitive;

    #[test]
    fn ignores_nonlocal_player_cameras() {
        let mut world = World::new();
        let remote = engine::ecs::player::spawn_local_player(&mut world);
        world.entity_mut(remote).remove::<LocalPlayer>();
        assert!(extract_camera(&mut world, 1.0).is_err());

        let local = engine::ecs::player::spawn_local_player(&mut world);
        let expected = extract_camera(&mut world, 1.0).unwrap();
        world.get_mut::<Transform>(remote).unwrap().translation = [99.0; 3];
        world.get_mut::<Camera>(remote).unwrap().near = -1.0;
        assert_eq!(extract_camera(&mut world, 1.0).unwrap(), expected);
        world.entity_mut(local).remove::<Player>();
        assert!(extract_camera(&mut world, 1.0).is_err());
    }

    #[test]
    fn camera_uniform_follows_ecs_changes() {
        let mut world = World::new();
        assert!(extract_camera(&mut world, 1.0).is_err());
        let entity = engine::ecs::player::spawn_local_player(&mut world);
        let camera = extract_camera(&mut world, 2.0).unwrap();
        assert_eq!(&camera[..4], &[3.0, 3.0, 5.0, 0.0]);
        assert!((camera[8] - 3.0_f32.sqrt()).abs() < 0.00001);
        assert_eq!(&camera[9..], &[2.0, 0.1, 100.0]);

        world.get_mut::<Transform>(entity).unwrap().translation = [0.0, 0.0, 5.0];
        world.get_mut::<Transform>(entity).unwrap().rotation = [0.0, 0.0, 0.0, 2.0];
        world.get_mut::<Camera>(entity).unwrap().fov_y_radians = std::f32::consts::FRAC_PI_2;
        world.get_mut::<Camera>(entity).unwrap().near = 1.0;
        world.get_mut::<Camera>(entity).unwrap().far = 10.0;
        let camera = extract_camera(&mut world, 1.0).unwrap();
        assert_eq!(&camera[..8], &[0.0, 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        assert!((camera[8] - 1.0).abs() < 0.00001);
        assert_eq!(&camera[9..], &[1.0, 1.0, 10.0]);

        world.get_mut::<Camera>(entity).unwrap().near = 10.0;
        assert!(extract_camera(&mut world, 1.0).is_err());
        world.get_mut::<Camera>(entity).unwrap().near = 1.0;
        world.get_mut::<Transform>(entity).unwrap().rotation = [0.0; 4];
        assert!(extract_camera(&mut world, 1.0).is_err());
        world.get_mut::<Transform>(entity).unwrap().rotation = [0.0, 0.0, 0.0, 1.0];
        let second = engine::ecs::player::spawn_local_player(&mut world);
        assert!(extract_camera(&mut world, 1.0).is_err());
        world.despawn(second);
        world.entity_mut(entity).remove::<Transform>();
        assert!(extract_camera(&mut world, 1.0).is_err());
    }

    #[test]
    fn mesh_batches_follow_ecs_changes() {
        let mut world = World::new();
        let mesh = Mesh::Primitive(MeshPrimitive::Cube);
        assert_eq!(extract_meshes(&mut world), HashMap::new());
        world.spawn(mesh);
        assert_eq!(extract_meshes(&mut world), HashMap::new());

        let cube = engine::ecs::scene::spawn_cube(&mut world);
        world.get_mut::<Transform>(cube).unwrap().translation = [1.0, 2.0, 3.0];
        world.get_mut::<Transform>(cube).unwrap().rotation = [0.0, 0.0, 1.0, 0.0];
        world.get_mut::<Transform>(cube).unwrap().scale = [2.0, 3.0, 4.0];
        world.get_mut::<Material>(cube).unwrap().color = [0.0, 1.0, 0.0];
        let expected: Vec<u8> = [
            1.0_f32, 2.0, 3.0, 0.0, 0.0, 1.0, 0.0, 2.0, 3.0, 4.0, 0.0, 1.0, 0.0,
        ]
        .into_iter()
        .flat_map(f32::to_le_bytes)
        .collect();
        assert_eq!(
            extract_meshes(&mut world),
            HashMap::from([(mesh, expected.clone())])
        );

        let second = engine::ecs::scene::spawn_cube(&mut world);
        let batches = extract_meshes(&mut world);
        assert_eq!(batches.len(), 1);
        assert_eq!(batches[&mesh].len(), 2 * 52);
        world.entity_mut(second).remove::<Material>();
        assert_eq!(
            extract_meshes(&mut world),
            HashMap::from([(mesh, expected)])
        );
        world.despawn(cube);
        assert_eq!(extract_meshes(&mut world), HashMap::new());
    }
}
