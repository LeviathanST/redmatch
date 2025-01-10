use bevy::{
    math::Vec3,
    prelude::{Query, Res, Resource, Transform, With},
    time::Time,
};
use bevy_rapier3d::{plugin::RapierContext, prelude::QueryFilter};

use crate::player::Player;

#[derive(Resource)]
pub struct Environment {
    pub gravity: f32,
}

pub fn apply_gravity(
    mut player_transform_query: Query<&mut Transform, With<Player>>,
    rapier_ctx: Res<RapierContext>,
    env: Res<Environment>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_transform_query.get_single_mut() {
        let ray_origin = player_transform.translation + Vec3::new(0., -0.5, 0.);
        let ray_direction = Vec3::new(0., -1., 0.);

        let hit = rapier_ctx.cast_ray(
            ray_origin,
            ray_direction,
            f32::MAX,
            true,
            QueryFilter::default(),
        );
    }
}
