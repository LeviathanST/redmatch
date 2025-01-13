use bevy::{
    input::ButtonInput,
    math::Vec3,
    prelude::{Component, Entity, KeyCode, Query, Res, Transform, With},
    time::Time,
};
use bevy_rapier3d::{
    plugin::RapierContext,
    prelude::{QueryFilter, Velocity},
};

use super::Player;

// TODO:
// + Velocity accelarate

#[derive(Component)]
pub struct PlayerMovement {
    /// (cm/s)
    /// **Default: 125.**
    pub speed: f32,
    /// (cm/s)
    pub jump_height: f32,
    pub jump_scale: f32,
    /// Whether player is on ground
    pub grounded: bool,
    /// Tolerance to ground detection
    /// The maximum allowable difference between player_y and ground_y.
    /// Briefly, player.y - ground.y is not greater than tolerance.
    /// **Default: 0.01**
    pub tolerance: f32,
    /// Which is limit time to fly before falling (delay time to fall)
    pub fly_time: f32,
    /// Which is counter time when flying
    pub fly_timer: f32,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        Self {
            speed: 125.,
            jump_height: 25.,
            jump_scale: 5.,
            grounded: false,
            tolerance: 0.01,
            fly_time: 0.25,
            fly_timer: 0.0,
        }
    }
}

pub fn controller(
    mut player_query: Query<(&mut PlayerMovement, &mut Velocity), With<Player>>,
    keycode_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (mut movement, mut velocity) in player_query.iter_mut() {
        if !movement.grounded {
            movement.fly_timer += dt;
            if movement.fly_timer > movement.fly_time {
                velocity.linvel.y += -9.8 * dt;
            }
        } else {
            velocity.linvel.y = 0.;
            movement.fly_timer = 0.;
            if keycode_input.pressed(KeyCode::Space) {
                velocity.linvel.y = movement.jump_height * dt * movement.jump_scale;
            }
        }

        let mut direction = Vec3::new(0., 0., 0.);
        if keycode_input.pressed(KeyCode::KeyW) {
            direction.z += -1.;
        }
        if keycode_input.pressed(KeyCode::KeyA) {
            direction.x += -1.;
        }
        if keycode_input.pressed(KeyCode::KeyS) {
            direction.z += 1.;
        }
        if keycode_input.pressed(KeyCode::KeyD) {
            direction.x += 1.;
        }

        if direction.length_squared() > 1e-6 {
            direction = direction.normalize();
        }

        velocity.linvel.x = direction.x * movement.speed * dt;
        velocity.linvel.z = direction.z * movement.speed * dt;
    }
}

pub fn check_ground_standing(
    mut player_query: Query<(Entity, &Transform, &mut PlayerMovement), With<Player>>,
    rapier_ctx: Res<RapierContext>,
) {
    for (entity, transform, mut movement) in player_query.iter_mut() {
        let filter = QueryFilter::new().exclude_collider(entity);

        let ground_cast = rapier_ctx.cast_ray_and_get_normal(
            transform.translation,
            Vec3::NEG_Y,
            1.5, // Range ()
            true,
            filter,
        );

        let grounded = if let Some((_ground_entity, intersection)) = ground_cast {
            (transform.translation.y - intersection.point.y).abs() <= movement.tolerance
        } else {
            false
        };

        movement.grounded = grounded;
    }
}
