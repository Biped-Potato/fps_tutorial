use bevy::prelude::*;

pub struct TracerPlugin;

impl Plugin for TracerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_tracers);
    }
}

#[derive(Component)]
pub struct BulletTracer {
    pub start_position: Vec3,
    pub end_position: Vec3,
    pub lifetime: f32,
    pub time_alive: f32,
}

impl BulletTracer {
    pub fn new(start: Vec3, end: Vec3, speed: f32) -> BulletTracer {
        BulletTracer {
            start_position: start,
            end_position: end,
            lifetime: Vec3::distance(start, end) / speed,
            time_alive: 0.,
        }
    }
}

fn update_tracers(
    mut commands: Commands,
    mut tracer_query: Query<(&mut BulletTracer, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    for (mut tracer, mut transform, entity) in tracer_query.iter_mut() {
        tracer.time_alive += time.delta_seconds();

        transform.translation = Vec3::lerp(
            tracer.start_position,
            tracer.end_position,
            f32::clamp(tracer.time_alive / tracer.lifetime, 0., 1.),
        );
        transform.look_at(tracer.end_position, Vec3::Y);

        if tracer.time_alive > tracer.lifetime {
            commands.entity(entity).despawn();
        }
    }
}
