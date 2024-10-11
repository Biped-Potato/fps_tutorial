use bevy::prelude::*;

pub mod game;

fn main() {
    App::new()
        .add_plugins((game::game::GamePlugin, DefaultPlugins))
        .run();
}
