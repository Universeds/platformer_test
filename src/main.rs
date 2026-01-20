use bevy::prelude::*;

mod game_plugin;
mod player;
mod physics;
mod camera;
mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(game_plugin::GamePlugin)
        .run();
}
