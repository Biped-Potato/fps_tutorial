use bevy::prelude::*;

pub mod game;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, game::game::GamePlugin))
        .run();
}
