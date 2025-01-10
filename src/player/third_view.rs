use bevy::{
    asset::Assets,
    color::Color,
    math::Vec2,
    pbr::{MaterialMeshBundle, StandardMaterial},
    prelude::{
        Camera3dBundle, Capsule3d, Commands, Mesh, MouseButton, OnEnter, Plugin, Res, ResMut,
        Transform,
    },
};

use bevy_third_person_camera::{
    Offset, ThirdPersonCamera, ThirdPersonCameraPlugin, ThirdPersonCameraTarget, Zoom,
};

use super::{camera_config::CameraConfig, Player, ViewMode};
pub struct ThirdViewPlugin;

impl Plugin for ThirdViewPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ThirdPersonCameraPlugin)
            .add_systems(OnEnter(ViewMode::ThirdPerson), setup);
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_config: Res<CameraConfig>,
) {
    let body_shape = meshes.add(Capsule3d::default());
    let body_material = materials.add(Color::BLACK);

    commands.spawn((
        Player,
        MaterialMeshBundle {
            mesh: body_shape,
            material: body_material,
            transform: Transform::from_xyz(0., 5., 0.),
            ..Default::default()
        },
        ThirdPersonCameraTarget,
    ));

    commands.spawn((
        Camera3dBundle {
            ..Default::default()
        },
        ThirdPersonCamera {
            // Cursor
            cursor_lock_active: true,
            // Camera
            offset_enabled: true,
            offset: Offset::new(0.5, 1.25),
            sensitivity: Vec2::splat(camera_config.sensitivity),
            aim_enabled: true,
            aim_zoom: 0.7,
            aim_speed: 3.,
            aim_button: MouseButton::Right,
            zoom: Zoom::new(5.0, 5.0),
            ..Default::default()
        },
    ));
}
