use bevy::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};

use super::{cursor::*, level::*, player::*, ui::*, window::*};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            cursor::CursorPlugin,
            player::PlayerPlugin,
            level::LevelPlugin,
            window::WindowSettingsPlugin,
            ui::UIPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
        ));
    }
}
