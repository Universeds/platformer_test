use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CharacterController {
    pub move_speed: f32,
    pub jump_force: f32,
    pub is_grounded: bool,
    pub acceleration: f32,
    pub deceleration: f32,
    pub air_acceleration: f32,
    pub air_deceleration: f32,
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            move_speed: 8.0,
            jump_force: 6.0,
            is_grounded: false,
            acceleration: 50.0,
            deceleration: 40.0,
            air_acceleration: 25.0,
            air_deceleration: 10.0,
        }
    }
}

#[derive(Component, Default)]
pub struct InputState {
    pub move_direction: Vec2,
    pub jump_pressed: bool,
}
