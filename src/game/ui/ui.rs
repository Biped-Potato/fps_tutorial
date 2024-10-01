use super::crosshair;
use bevy::prelude::*;
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, crosshair::spawn_crosshair);
    }
}
