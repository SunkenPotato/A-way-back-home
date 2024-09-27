pub mod player;

use bevy::{app::App, asset::AssetServer, prelude::{Camera2d, Commands, Res}, DefaultPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
}