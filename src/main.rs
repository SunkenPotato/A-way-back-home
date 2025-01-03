#![allow(unused_parens)]

pub mod components;
pub mod macros;
pub mod player;
pub mod render;
pub mod utils;
pub mod world;

use avian2d::PhysicsPlugins;
use bevy::{
    app::{App, FixedUpdate},
    prelude::{ImagePlugin, PluginGroup},
    utils::default,
    window::{Window, WindowPlugin},
};
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;
use components::ComponentPlugin;
use player::PlayerPlugin;
use render::RenderPlugins;
use world::WorldPlugins;

fn main() {
    App::new()
        .add_plugins(
            bevy::DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: bevy::window::WindowMode::BorderlessFullscreen(
                            bevy::window::MonitorSelection::Primary,
                        ),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(LdtkPlugin)
        .add_plugins(RenderPlugins)
        .add_plugins(WorldPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(ComponentPlugin)
        .add_plugins((
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian2dPlugin::new(FixedUpdate),
            PhysicsPlugins::new(FixedUpdate),
            #[cfg(debug_assertions)]
            avian2d::prelude::PhysicsDebugPlugin::default(),
        ))
        .run();
}
