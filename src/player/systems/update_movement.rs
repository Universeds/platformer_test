use bevy::prelude::*;
use avian3d::prelude::*;

use crate::player::{CharacterController, InputState};
use crate::camera::MainCamera;

fn update_linear_movement() {
    println!("Updating linear movement...");
}

fn update_grounded(
    controller: &mut CharacterController,
    spatial_query: &SpatialQuery,
    entity: Entity,
    transform: &Transform,
) {
    let ray_origin = transform.translation;
    let ray_direction = Dir3::NEG_Y;
    let max_distance = 1.2;

    let filter = SpatialQueryFilter::from_excluded_entities([entity]);

    controller.is_grounded = spatial_query
        .cast_ray(
            ray_origin,
            ray_direction,
            max_distance,
            true,
            &filter,
        )
        .is_some();
}



pub fn update_movement(
    _time: Res<Time>,
    mut query: Query<(Entity, &InputState, &mut CharacterController, &mut LinearVelocity, &Transform)>,
    camera_query: Query<&Transform, (With<MainCamera>, Without<InputState>)>,
    spatial_query: SpatialQuery,
) {
    let camera_transform = match camera_query.single() {
        Ok(t) => t,
        Err(_) => return,
    };
    let camera_forward = camera_transform.forward();
    let camera_right = camera_transform.right();
    
    let flatten_forward = Vec3::new(camera_forward.x, 0.0, camera_forward.z).normalize_or_zero();
    let flatten_right = Vec3::new(camera_right.x, 0.0, camera_right.z).normalize_or_zero();
    
    for (entity, input, mut controller, mut velocity, transform) in &mut query {
        update_grounded(&mut controller, &spatial_query, entity, transform);

        let move_direction = flatten_forward * input.move_direction.y + flatten_right * input.move_direction.x;
        let move_vec = move_direction * controller.move_speed;
        
        velocity.x = move_vec.x;
        velocity.z = move_vec.z;
        
        if input.jump_pressed && controller.is_grounded {
            velocity.y = controller.jump_force;
        }
    }
}
