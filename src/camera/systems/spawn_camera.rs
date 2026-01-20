use bevy::prelude::*;
use crate::camera::MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
