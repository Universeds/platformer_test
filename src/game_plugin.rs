use bevy::prelude::*;
use avian3d::prelude::*;

use crate::player::PlayerPluginStruct;
use crate::physics::PhysicsPluginStruct;
use crate::camera::CameraPluginStruct;
use crate::ui::UIPluginStruct;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PhysicsPlugins::default())
            .add_plugins(PhysicsPluginStruct)
            .add_plugins(PlayerPluginStruct)
            .add_plugins(CameraPluginStruct)
            .add_plugins(UIPluginStruct);
    }
}
