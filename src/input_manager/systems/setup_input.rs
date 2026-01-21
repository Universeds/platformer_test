use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

fn spawn_player(mut commands: Commands) {
    commands.entity()
}

#[derive(Component)]
struct Player;

#[derive(InputAction)]
#[action_output(Vec2)]
struct Movement;
