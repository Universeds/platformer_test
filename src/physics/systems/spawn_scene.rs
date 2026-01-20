use bevy::prelude::*;
use avian3d::prelude::*;

pub fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(50.0, 0.5, 50.0),
        Mesh3d(meshes.add(Cuboid::new(50.0, 0.5, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
        Transform::from_xyz(0.0, -0.5, 0.0),
    ));
    commands.spawn((
    DirectionalLight {
        illuminance: 10000.0,
        shadows_enabled: true,
        ..default()
    },
    Transform::from_rotation(Quat::from_euler(
        EulerRot::XYZ,
        -std::f32::consts::FRAC_PI_4,
        std::f32::consts::FRAC_PI_4,
        0.0,
    )),
));
}
