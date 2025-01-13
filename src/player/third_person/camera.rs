use bevy::prelude::{Query, Transform, With, Without};

use crate::player::camera::{Camera, CameraTarget};

pub fn update(
    mut camera_q: Query<(&mut Transform, &Camera), Without<CameraTarget>>,
    target_q: Query<&Transform, With<CameraTarget>>,
) {
    for target_transform in target_q.iter() {
        for (mut camera_transform, camera) in camera_q.iter_mut() {
            camera_transform.translation = target_transform.translation + camera.offset;
        }
    }
}
