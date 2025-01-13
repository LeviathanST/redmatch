// NOTE: Inspired by bevy_third_person_camera package
// https://github.com/The-DevBlog/bevy_third_person_camera
// The purpose of reinventing the wheel is to learn
// Add more feature like first person
use bevy::{
    input::ButtonInput,
    math::Vec3,
    prelude::{Component, KeyCode, NextState, Res, ResMut, State},
};

use super::ViewMode;

#[derive(Component)]
pub struct CameraTarget;

#[derive(Component)]
pub struct Camera {
    /// Offset coordinate between player and camera
    /// x: right, -x: left
    /// y: up,    -y: down
    /// z: back,  -z: forwad
    pub offset: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0., 2., 5.),
        }
    }
}

pub fn toggle_view_mode(
    mut next_state: ResMut<NextState<ViewMode>>,
    input: Res<ButtonInput<KeyCode>>,
    curr_state: Res<State<ViewMode>>,
) {
    if input.just_pressed(KeyCode::KeyY) {
        match curr_state.get() {
            ViewMode::FirstPerson => next_state.set(ViewMode::ThirdPerson),
            ViewMode::ThirdPerson => next_state.set(ViewMode::FirstPerson),
        };
    }
}
