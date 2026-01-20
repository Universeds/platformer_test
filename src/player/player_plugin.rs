use bevy::prelude::*;

use super::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, (read_input, move_player).chain());
    }
}
