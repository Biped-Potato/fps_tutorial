use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
pub struct CameraController {
    pub rotation: Vec2,
    pub rotation_lock: f32,
    pub sensitivity: f32,
}

pub fn update_camera_controller(
    mut mouse_motion: EventReader<MouseMotion>,
    mut camera_query: Query<(&mut CameraController, &mut Transform)>,
) {
    if let Ok((mut camera_controller, mut transform)) = camera_query.get_single_mut() {
        for ev in mouse_motion.read() {
            camera_controller.rotation.y -= ev.delta.x * camera_controller.sensitivity;
            camera_controller.rotation.x -= ev.delta.y * camera_controller.sensitivity;

            camera_controller.rotation.x = f32::clamp(
                camera_controller.rotation.x,
                -camera_controller.rotation_lock,
                camera_controller.rotation_lock,
            );
        }
        let y_quat = Quat::from_axis_angle(Vec3::Y, camera_controller.rotation.y.to_radians());
        let x_quat = Quat::from_axis_angle(Vec3::X, camera_controller.rotation.x.to_radians());
        transform.rotation = y_quat * x_quat;
    }
}
