use bevy_ecs::prelude::{Component, Entity, World};

use super::scene::{Camera, Transform};

/// Identifies a player entity.
#[derive(Component, Debug)]
pub struct Player;

/// Marks the player controlled by this client and supplying its active camera.
#[derive(Component, Debug)]
pub struct LocalPlayer;

/// Spawns the local player with a fixed camera at (3, 3, 5), looking at the origin.
pub fn spawn_local_player(world: &mut World) -> Entity {
    let yaw = 3.0_f32.atan2(5.0);
    let pitch = -3.0_f32.atan2(34.0_f32.sqrt());
    let (sin_yaw, cos_yaw) = (yaw / 2.0).sin_cos();
    let (sin_pitch, cos_pitch) = (pitch / 2.0).sin_cos();
    world
        .spawn((
            Player,
            LocalPlayer,
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
    use bevy_ecs::prelude::With;

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
