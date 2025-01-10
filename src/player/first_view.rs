use std::f32::consts::TAU;

use bevy::{
    asset::Assets,
    pbr::StandardMaterial,
    prelude::*,
    render::camera::Exposure,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_fps_controller::controller::{
    CameraConfig, FpsController, FpsControllerInput, FpsControllerPlugin, LogicalPlayer,
    RenderPlayer,
};
use bevy_rapier3d::prelude::*;

use super::ViewMode;
pub struct FirstViewPlugin;

impl Plugin for FirstViewPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(FpsControllerPlugin)
            .add_systems(OnEnter(ViewMode::FirstPerson), setup)
            .add_systems(
                Update,
                manage_cursor.run_if(in_state(ViewMode::FirstPerson)),
            );
    }
}

// TODO:
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let height = 3.0;
    let logical_entity = commands
        .spawn((
            Collider::cylinder(height / 2.0, 0.5),
            // A capsule can be used but is NOT recommended
            // If you use it, you have to make sure each segment point is
            // equidistant from the translation of the player transform
            // Collider::capsule_y(height / 2.0, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0., 0., 0.))),
            LogicalPlayer,
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                air_acceleration: 80.0,
                ..default()
            },
        ))
        .insert(CameraConfig { height_offset: 0. })
        .id();

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: TAU / 5.0,
                ..default()
            }),
            ..default()
        },
        RenderPlayer { logical_entity },
    ));
}
fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut controller_query: Query<&mut FpsController>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        if btn.just_pressed(MouseButton::Left) {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
            for mut controller in &mut controller_query {
                controller.enable_input = true;
            }
        }
    }
}
