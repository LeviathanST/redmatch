mod target;

use bevy::{
    app::{Startup, Update},
    asset::Assets,
    color::Color,
    math::{Vec2, Vec3},
    pbr::{
        light_consts::lux::OVERCAST_DAY, DirectionalLight, DirectionalLightBundle, PbrBundle,
        StandardMaterial,
    },
    prelude::{Commands, Component, Cuboid, Mesh, Plane3d, Plugin, ResMut, Transform},
};
use bevy_rapier3d::prelude::Collider;

#[derive(Component)]
pub struct Ground;

pub struct MapTestPlugin;

impl Plugin for MapTestPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(1000.)));
    let material = materials.add(Color::WHITE);

    // Plane

    commands.spawn((
        Ground,
        Collider::cuboid(500., 1., 500.),
        PbrBundle {
            mesh: floor,
            transform: Transform::IDENTITY,
            material: material.clone(),
            ..Default::default()
        },
    ));

    // Wall
    let cube = meshes.add(Cuboid::from_length(30.));
    commands.spawn((
        Collider::cuboid(15., 15., 15.),
        PbrBundle {
            mesh: cube,
            transform: Transform::from_xyz(0., 0., -100.),
            material,
            ..Default::default()
        },
    ));

    // Lighting
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: OVERCAST_DAY,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(100., 200., 100.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
