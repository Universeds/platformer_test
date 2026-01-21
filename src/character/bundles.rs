use bevy::prelude::*;
use super::components::*;

#[derive(Bundle)]
pub struct BaseCharacterBundle {
    pub character: Character,
    pub health: Health,
    pub move_speed: MoveSpeed,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
