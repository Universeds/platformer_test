use bevy::prelude::*;
use crate::player::InputState;

pub fn read_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut InputState>,
) {
    for mut input in &mut query {
        let mut direction = Vec2::ZERO;
        
        if keyboard.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        
        input.move_direction = if direction.length_squared() > 0.0 {
            direction.normalize()
        } else {
            Vec2::ZERO
        };
        
        input.jump_pressed = keyboard.pressed(KeyCode::Space);
    }
}
