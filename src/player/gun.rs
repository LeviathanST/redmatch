use bevy::{
    asset::Assets,
    color::Color,
    input::ButtonInput,
    math::{Vec2, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    prelude::{
        Camera, Commands, Component, Cuboid, Entity, GlobalTransform, Mesh, MouseButton, Query,
        Res, ResMut, Resource, Transform, With,
    },
    time::{Time, Timer},
    window::{PrimaryWindow, Window},
};
use bevy_rapier3d::{plugin::RapierContext, prelude::QueryFilter};

pub fn shoot(
    mut commands: Commands,
    mut window: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut GlobalTransform, &mut Camera, &mut Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cooldown: ResMut<FireRateCooldown>,
    rapier_ctx: Res<RapierContext>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    cooldown.timer.tick(time.delta());

    let window = window.get_single_mut().unwrap();
    if let Ok((global_transform, camera, third_person_transform)) = camera_query.get_single_mut() {
        if cooldown.timer.finished() && mouse_input.pressed(MouseButton::Left) {
            if time.elapsed_seconds() >= 0.25 {
                let Some(ray) = camera.viewport_to_world(
                    &global_transform,
                    Vec2::new(window.width() / 2., window.height() / 2.),
                ) else {
                    return;
                };

                let hit = rapier_ctx.cast_ray_and_get_normal(
                    ray.origin,
                    ray.direction.into(),
                    f32::MAX,
                    true,
                    QueryFilter::default(),
                );

                if let Some((_entity, ray_intersection)) = hit {
                    let material = StandardMaterial {
                        base_color: Color::srgb(1., 1., 0.),
                        unlit: true,
                        ..Default::default()
                    };

                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Cuboid::from_size(Vec3::new(0.1, 0.1, 1.))),
                            material: materials.add(material),
                            ..Default::default()
                        },
                        BulletTracer::new(
                            third_person_transform.translation,
                            ray_intersection.point,
                            0.1,
                            100.,
                        ),
                    ));
                }

                cooldown.timer.reset();
            }
        }
    }
}

#[derive(Component)]
pub struct BulletTracer {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub cool_down: f32,
    pub lifetime: f32,
    pub time_alive: f32,
}

impl BulletTracer {
    pub fn new(start_pos: Vec3, end_pos: Vec3, cool_down: f32, speed: f32) -> Self {
        Self {
            start_pos,
            end_pos,
            cool_down,
            lifetime: Vec3::distance(start_pos, end_pos) / speed,
            time_alive: 0.,
        }
    }
}

pub fn update_tracer(
    mut commands: Commands,
    mut tracer_query: Query<(&mut BulletTracer, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut tracer, mut transform, entity) in tracer_query.iter_mut() {
        tracer.time_alive += time.delta_seconds();

        transform.translation = Vec3::lerp(
            tracer.start_pos,
            tracer.end_pos,
            f32::clamp(tracer.time_alive / tracer.lifetime, 0., 1.),
        );
        transform.look_at(tracer.end_pos, Vec3::Y);

        if tracer.time_alive > tracer.lifetime {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Resource)]
pub struct FireRateCooldown {
    pub timer: Timer,
}
