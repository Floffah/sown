//! Scene components and a directly authored single-cube scene.

use bevy_ecs::prelude::{Component, Entity, World};

#[derive(Component, Debug, PartialEq)]
pub struct Transform {
    pub translation: [f32; 3],
    /// Quaternion in x, y, z, w order.
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Component, Debug, PartialEq, Eq)]
pub enum Mesh {
    /// A unit cube centred on the origin.
    Cube,
}

#[derive(Component, Debug, PartialEq)]
pub struct Material {
    pub color: [f32; 3],
}

/// Spawns a unit cube at the origin. Call on an empty world for a single-cube scene.
pub fn spawn_cube(world: &mut World) -> Entity {
    world
        .spawn((
            Transform {
                translation: [0.0; 3],
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0; 3],
            },
            Mesh::Cube,
            Material {
                color: [0.9, 0.3, 0.15],
            },
        ))
        .id()
}

#[cfg(test)]
mod tests {
    use bevy_ecs::prelude::With;

    use super::*;

    #[test]
    fn scene_contains_one_cube_with_transform_and_material() {
        let mut world = World::new();
        let cube = spawn_cube(&mut world);

        println!("{:#?}", world.entities());
        let meshes: Vec<Entity> = world
            .query_filtered::<Entity, With<Mesh>>()
            .iter(&world)
            .collect();
        assert_eq!(meshes, [cube]);
        assert_eq!(world.get::<Mesh>(cube), Some(&Mesh::Cube));
        assert_eq!(
            world.get::<Transform>(cube),
            Some(&Transform {
                translation: [0.0; 3],
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0; 3],
            })
        );
        assert_eq!(
            world.get::<Material>(cube),
            Some(&Material {
                color: [0.9, 0.3, 0.15],
            })
        );
    }
}
