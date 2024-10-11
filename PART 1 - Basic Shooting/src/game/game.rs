use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};

use super::{level::level, player::player, ui::ui, window::window};
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            level::LevelPlugin,
            player::PlayerPlugin,
            window::WindowSettingsPlugin,
            ui::UiPlugin,
        ));
    }
}
