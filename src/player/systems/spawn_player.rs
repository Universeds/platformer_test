use bevy::prelude::*;
use avian3d::prelude::*;

use crate::player::{Player, CharacterController, InputState};

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        CharacterController::default(),
        InputState::default(),
        RigidBody::Dynamic,
        Collider::capsule(0.5, 1.0),
        LockedAxes::ROTATION_LOCKED,
        Mesh3d(meshes.add(Capsule3d::new(0.5, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.8))),
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));
}
