use bevy::prelude::*;
use crate::camera::MainCamera;
use crate::player::Player;

pub fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            let target = player_transform.translation + Vec3::new(0.0, 10.0, 15.0);
            camera_transform.translation = camera_transform.translation.lerp(target, 1.0);
            camera_transform.look_at(player_transform.translation, Vec3::Y);
        }
    }
}
