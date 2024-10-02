use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{plugin::RapierContext, prelude::QueryFilter};

use crate::game::{level::targets::{DeadTarget, Target}, shooting::tracer::{BulletTracer, TracerPlugin}};

use super::camera_controller::{self, CameraController};
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TracerPlugin)
            .add_systems(Startup, init_player)
            .add_systems(
                Update,
                (update_player, camera_controller::update_camera_controller),
            );
    }
}
#[derive(Component)]
pub struct Player {}
fn init_player(mut commands: Commands) {
    let fov = 103.0;
    commands.spawn((
        CameraController {
            sensitivity: 0.036,
            rotation: Vec2::ZERO,
            rotation_lock: 88.0,
        },
        Player {},
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 10., 0.)),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: (fov / 360.0) * (std::f32::consts::PI * 2.0),
                ..Default::default()
            }),
            ..Default::default()
        },
    ));
}

fn update_player(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut player_query: Query<(
        &mut Player,
        &mut Transform,
        &mut GlobalTransform,
        &mut Camera,
    )>,
    target_query : Query<Entity,With<Target>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok((_player, transform, global_transform, camera)) = player_query.get_single_mut() {
        if mouse_input.just_pressed(MouseButton::Left) {
            let Some(ray) = camera.viewport_to_world(
                &global_transform,
                Vec2::new(window.width() / 2., window.height() / 2.),
            ) else {
                return;
            };
            let hit = rapier_context.cast_ray_and_get_normal(
                ray.origin,
                ray.direction.into(),
                f32::MAX,
                true,
                QueryFilter::default(),
            );
            if let Some((entity, ray_intersection)) = hit {
                if let Ok(_entity) = target_query.get(entity)
                {
                    commands.entity(entity).insert(DeadTarget);
                } 
                let tracer_material = StandardMaterial {
                    base_color: Color::srgb(1., 1., 0.),
                    unlit: true,
                    ..Default::default()
                };
                commands.spawn((
                    PbrBundle {
                        transform: Transform::from_xyz(0., 100000., 0.),
                        mesh: meshes.add(Cuboid::from_size(Vec3::new(0.1, 0.1, 1.))),
                        material: materials.add(tracer_material),
                        ..default()
                    },
                    BulletTracer::new(transform.translation, ray_intersection.point, 100.),
                ));
            }
        }
    }
}
