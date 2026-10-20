use bevy_ecs::prelude::{Component, Entity, Query, Res, Resource, With, World};

use super::{
    FrameTime,
    scene::{Camera, Transform},
};

const MOVEMENT_SPEED: f32 = 3.0;

/// Identifies a player entity.
#[derive(Component, Debug)]
pub struct Player;

/// Marks the player controlled by this client and supplying its active camera.
#[derive(Component, Debug)]
pub struct LocalPlayer;

/// Movement along world X and Z axes, supplied by the client chrome.
/// Values are clamped to [-1, 1] and combined magnitude is limited to one.
/// Non-finite input is treated as no movement.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct MovementIntent {
    pub x: f32,
    pub z: f32,
}

pub(super) fn move_local_player(
    intent: Res<MovementIntent>,
    time: Res<FrameTime>,
    mut players: Query<&mut Transform, (With<Player>, With<LocalPlayer>)>,
) {
    let intent = intent.into_inner();
    let time = time.into_inner();
    if !intent.x.is_finite() || !intent.z.is_finite() {
        return;
    }
    let x = intent.x.clamp(-1.0, 1.0);
    let z = intent.z.clamp(-1.0, 1.0);
    let distance = MOVEMENT_SPEED * time.0.as_secs_f32() / x.hypot(z).max(1.0);
    for mut transform in &mut players {
        transform.translation[0] += x * distance;
        transform.translation[2] += z * distance;
    }
}

/// Spawns the local player with its camera at (3, 3, 5), looking at the origin.
pub fn spawn_local_player(world: &mut World) -> Entity {
    let entity = spawn_player(world);
    world.entity_mut(entity).insert(LocalPlayer);
    entity
}

/// Spawns a player with its camera at (3, 3, 5), looking at the origin.
pub fn spawn_player(world: &mut World) -> Entity {
    let yaw = 3.0_f32.atan2(5.0);
    let pitch = -3.0_f32.atan2(34.0_f32.sqrt());
    let (sin_yaw, cos_yaw) = (yaw / 2.0).sin_cos();
    let (sin_pitch, cos_pitch) = (pitch / 2.0).sin_cos();
    world
        .spawn((
            Player,
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
    use super::*;
    use crate::ecs::Ecs;
    use bevy_ecs::prelude::With;
    use std::time::Duration;

    fn displacement(intent: MovementIntent, frames: u32) -> [f32; 3] {
        let mut ecs = Ecs::default();
        let player = spawn_local_player(&mut ecs.world);
        ecs.world.insert_resource(intent);
        for _ in 0..frames {
            ecs.tick(Duration::from_secs(1) / frames);
        }
        let position = ecs.world.get::<Transform>(player).unwrap().translation;
        [position[0] - 3.0, position[1] - 3.0, position[2] - 5.0]
    }

    #[test]
    fn movement_preserves_analog_strength_and_limits_diagonals() {
        for (intent, expected) in [
            (MovementIntent::default(), [0.0, 0.0, 0.0]),
            (MovementIntent { x: 0.0, z: -1.0 }, [0.0, 0.0, -3.0]),
            (MovementIntent { x: -1.0, z: 0.0 }, [-3.0, 0.0, 0.0]),
            (MovementIntent { x: 0.5, z: 0.0 }, [1.5, 0.0, 0.0]),
            (MovementIntent { x: 100.0, z: 0.0 }, [3.0, 0.0, 0.0]),
        ] {
            assert_eq!(displacement(intent, 1), expected);
        }
        let diagonal = displacement(MovementIntent { x: 1.0, z: -1.0 }, 1);
        assert!((diagonal[0].hypot(diagonal[2]) - 3.0).abs() < 0.0001);
        assert!(diagonal[0] > 0.0 && diagonal[2] < 0.0);
    }

    #[test]
    fn movement_uses_elapsed_time_and_rejects_non_finite_input() {
        let intent = MovementIntent { x: 1.0, z: 0.0 };
        let single = displacement(intent, 1);
        let multiple = displacement(intent, 60);
        for axis in 0..3 {
            assert!((single[axis] - multiple[axis]).abs() < 0.0001);
        }
        for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(
                displacement(MovementIntent { x: invalid, z: 1.0 }, 1),
                [0.0; 3]
            );
            assert_eq!(
                displacement(MovementIntent { x: 1.0, z: invalid }, 1),
                [0.0; 3]
            );
        }
    }

    #[test]
    fn movement_only_changes_local_player_translation() {
        let mut ecs = Ecs::default();
        let local = spawn_local_player(&mut ecs.world);
        let rotation = ecs.world.get::<Transform>(local).unwrap().rotation;
        let transform = || Transform {
            translation: [0.0; 3],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0; 3],
        };
        let remote = ecs.world.spawn((Player, transform())).id();
        let mesh = ecs.world.spawn(transform()).id();
        let marker_only = ecs.world.spawn((LocalPlayer, transform())).id();
        ecs.world.insert_resource(MovementIntent { x: 1.0, z: 1.0 });
        ecs.tick(Duration::ZERO);
        assert_eq!(
            ecs.world.get::<Transform>(local).unwrap().translation,
            [3.0, 3.0, 5.0]
        );
        ecs.tick(Duration::from_secs(1));
        let transform = ecs.world.get::<Transform>(local).unwrap();
        assert_eq!(transform.translation[1], 3.0);
        assert_eq!(transform.rotation, rotation);
        assert_eq!(transform.scale, [1.0; 3]);
        for entity in [remote, mesh, marker_only] {
            assert_eq!(
                ecs.world.get::<Transform>(entity).unwrap().translation,
                [0.0; 3]
            );
        }
    }

    #[test]
    fn player_and_camera_share_one_entity() {
        let mut world = World::new();
        let player = spawn_local_player(&mut world);
        let players: Vec<Entity> = world
            .query_filtered::<Entity, (With<Player>, With<LocalPlayer>)>()
            .iter(&world)
            .collect();
        assert_eq!(players, [player]);
        assert_eq!(
            world.get::<Transform>(player).unwrap().translation,
            [3.0, 3.0, 5.0]
        );
        assert_eq!(world.get::<Transform>(player).unwrap().scale, [1.0; 3]);
        assert_eq!(
            world.get::<Camera>(player),
            Some(&Camera {
                fov_y_radians: 60.0_f32.to_radians(),
                near: 0.1,
                far: 100.0,
            })
        );
    }
}
