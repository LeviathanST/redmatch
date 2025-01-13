mod camera;
mod first_person;
mod gun;
mod movement;
mod third_person;

use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    color::Color,
    pbr::{MaterialMeshBundle, StandardMaterial},
    prelude::{
        AppExtStates, Camera3dBundle, Capsule3d, Commands, Component, IntoSystemConfigs, Mesh,
        ResMut, States, Transform,
    },
    time::{Timer, TimerMode},
};

use bevy_rapier3d::prelude::{RigidBody, Velocity};
use camera::{Camera, CameraTarget};
use first_person::FirstPersonPlugin;
use gun::FireRateCooldown;
use movement::PlayerMovement;
use third_person::ThirdPersonPlugin;

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
        app.insert_state(ViewMode::FirstPerson)
            .insert_resource(FireRateCooldown {
                timer: Timer::from_seconds(0.25, TimerMode::Once),
            })
            .add_plugins(ThirdPersonPlugin)
            .add_plugins(FirstPersonPlugin)
            .add_systems(Startup, spawn_player_entity)
            .add_systems(Update, camera::toggle_view_mode)
            .add_systems(Update, (gun::shoot, gun::update_tracer))
            .add_systems(
                Update,
                (movement::check_ground_standing, movement::controller).chain(),
            );
    }
}

fn spawn_player_entity(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body_shape = meshes.add(Capsule3d::new(0.5, 1.));
    let body_material = materials.add(Color::BLACK);

    // Player shape
    commands.spawn((
        Player,
        MaterialMeshBundle {
            mesh: body_shape,
            material: body_material,
            transform: Transform::from_xyz(0., 5., 0.),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Velocity::zero(),
        PlayerMovement {
            ..Default::default()
        },
        CameraTarget,
    ));

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2., 5.),
            ..Default::default()
        },
        Camera {
            ..Default::default()
        },
    ));
}
