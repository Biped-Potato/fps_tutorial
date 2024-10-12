use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{camera_controller, input::*, player_movement::*, player_shooting::update_player};
use crate::game::shooting;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(shooting::tracer::TracerPlugin)
            .init_resource::<PlayerInput>()
            .add_systems(
                Update,
                (
                    update_movement_input,
                    update_player, 
                    camera_controller::update_camera_controller
                ),
            )
            //physics timestep
            .add_systems(FixedUpdate, update_movement)
            .add_systems(Startup, init_player);
    }
}

#[derive(Component)]
pub struct Player {
    pub velocity : Vec3,
    pub gravity : f32,
    pub speed : f32,
}
fn init_player(mut commands: Commands) {
    let fov = 103.0_f32.to_radians();
    let camera_entity = commands.spawn((
        Camera3dBundle {
            transform: Transform::IDENTITY,
            projection: Projection::Perspective(PerspectiveProjection {
                fov: fov,
                ..default()
            }),
            ..default()
        },
        camera_controller::CameraController {
            sensitivity: 0.035,
            rotation: Vec2::ZERO,
            rotation_lock: 88.0,
        },
    )).id();

    let player_entity = commands.spawn((
        Player {
            velocity : Vec3::ZERO,
            gravity : 9.8,
            speed : 20.0,
        },
        SpatialBundle{
            transform : Transform::from_translation(Vec3::new(0., 30., 0.)),
            ..Default::default()
        },
        Collider::cuboid(1.,10., 1.),
        RigidBody::KinematicPositionBased,
        KinematicCharacterController{
            up : Vec3::Y,
            offset : CharacterLength::Absolute(0.01),
            ..default()
        }
    )).id();

    commands.entity(player_entity).add_child(camera_entity);
}

