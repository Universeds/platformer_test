use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use super::systems::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Startup, spawn_fps_counter)
            .add_systems(Update, update_fps_counter);
    }
}
