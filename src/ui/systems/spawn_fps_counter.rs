use bevy::prelude::*;
use crate::ui::components::FpsText;

pub fn spawn_fps_counter(mut commands: Commands) {
    commands.spawn((
        Text::new("FPS: 0"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        FpsText,
    ));
}
