mod camera_config;
mod first_view;
mod gun;
mod third_view;

use bevy::{
    app::{Plugin, PreUpdate, Update},
    prelude::{AppExtStates, Component, States},
    time::{Timer, TimerMode},
};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use camera_config::CameraConfig;

use first_view::FirstViewPlugin;
use gun::FireRateCooldown;
use third_view::ThirdViewPlugin;

#[derive(States, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum ViewMode {
    ThirdPerson,
    FirstPerson,
}

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_state(ViewMode::FirstPerson)
            .insert_resource(CameraConfig { sensitivity: 0.7 })
            .insert_resource(FireRateCooldown {
                timer: Timer::from_seconds(0.25, TimerMode::Once),
            })
            .add_plugins(FirstViewPlugin)
            .add_plugins(ThirdViewPlugin)
            .add_systems(Update, (gun::shoot, gun::update_tracer));
    }
}
