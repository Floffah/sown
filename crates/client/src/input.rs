use engine::ecs::player::MovementIntent;
use winit::keyboard::KeyCode;

#[derive(Default)]
#[allow(clippy::struct_excessive_bools)] // Each movement key can be held independently.
pub(super) struct MovementKeys {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
}

impl MovementKeys {
    pub fn set(&mut self, code: KeyCode, pressed: bool) {
        match code {
            KeyCode::KeyW => self.forward = pressed,
            KeyCode::KeyS => self.backward = pressed,
            KeyCode::KeyA => self.left = pressed,
            KeyCode::KeyD => self.right = pressed,
            _ => {}
        }
    }

    pub fn intent(&self) -> MovementIntent {
        MovementIntent {
            x: f32::from(self.right) - f32::from(self.left),
            z: f32::from(self.backward) - f32::from(self.forward),
        }
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_held_keys_cancels_opposites_and_clears() {
        let mut keys = MovementKeys::default();
        for (key, expected) in [
            (KeyCode::KeyW, MovementIntent { x: 0.0, z: -1.0 }),
            (KeyCode::KeyS, MovementIntent { x: 0.0, z: 1.0 }),
            (KeyCode::KeyA, MovementIntent { x: -1.0, z: 0.0 }),
            (KeyCode::KeyD, MovementIntent { x: 1.0, z: 0.0 }),
        ] {
            keys.set(key, true);
            keys.set(key, true);
            assert_eq!(keys.intent(), expected);
            keys.set(key, false);
            assert_eq!(keys.intent(), MovementIntent::default());
        }
        for key in [KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD] {
            keys.set(key, true);
        }
        assert_eq!(keys.intent(), MovementIntent::default());
        keys.set(KeyCode::KeyS, false);
        keys.set(KeyCode::KeyA, false);
        assert_eq!(keys.intent(), MovementIntent { x: 1.0, z: -1.0 });
        keys.clear();
        assert_eq!(keys.intent(), MovementIntent::default());
    }
}
