//! Scene components and directly authored prototype entities.

use bevy_ecs::prelude::{Component, Entity, World};
pub use project::world::MeshPrimitive;
use project::world::NodeType;

/// Identity and type of an authored world node.
#[derive(Component, Debug)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub kind: NodeType,
}

/// The authored transform relative to the node's parent.
#[derive(Component, Debug)]
pub struct LocalTransform(pub Transform);

/// World-space transform consumed by rendering.
#[derive(Component, Debug, PartialEq)]
pub struct Transform {
    pub translation: [f32; 3],
    /// Quaternion in x, y, z, w order.
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mesh {
    /// Built-in primitive geometry shared with the source format.
    Primitive(MeshPrimitive),
}

#[derive(Component, Debug, PartialEq)]
pub struct Material {
    pub color: [f32; 3],
}

/// Spawns a unit cube at the origin
pub fn spawn_cube(world: &mut World) -> Entity {
    world
        .spawn((
            Transform {
                translation: [0.0; 3],
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0; 3],
            },
            Mesh::Primitive(MeshPrimitive::Cube),
            Material {
                color: [0.9, 0.3, 0.15],
            },
        ))
        .id()
}

/// A perspective camera looking along local -Z, with local +Y as up.
/// Position and orientation come from its `Transform`; scale is ignored.
#[derive(Component, Debug, PartialEq)]
pub struct Camera {
    pub fov_y_radians: f32,
    pub near: f32,
    pub far: f32,
}

/// Spawns the prototype camera at (3, 3, 5), looking at the origin.
pub fn spawn_camera(world: &mut World) -> Entity {
    let yaw = 3.0_f32.atan2(5.0);
    let pitch = -3.0_f32.atan2(34.0_f32.sqrt());
    let (sin_yaw, cos_yaw) = (yaw / 2.0).sin_cos();
    let (sin_pitch, cos_pitch) = (pitch / 2.0).sin_cos();
    world
        .spawn((
            Transform {
                translation: [3.0, 3.0, 5.0],
                // Compose yaw about Y followed by pitch about the camera's local X.
                rotation: [
                    cos_yaw * sin_pitch,
                    sin_yaw * cos_pitch,
                    -sin_yaw * sin_pitch,
                    cos_yaw * cos_pitch,
                ],
                scale: [1.0; 3],
            },
            Camera {
                fov_y_radians: 60.0_f32.to_radians(),
                near: 0.1,
                far: 100.0,
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
        assert_eq!(
            world.get::<Mesh>(cube),
            Some(&Mesh::Primitive(MeshPrimitive::Cube))
        );
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
