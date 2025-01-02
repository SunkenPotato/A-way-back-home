#![allow(unused_parens)]

pub mod macros;
pub mod player;
pub mod render;
pub mod utils;
pub mod world;

use avian2d::{prelude::PhysicsDebugPlugin, PhysicsPlugins};
use bevy::{
    app::{App, FixedUpdate},
    prelude::{ImagePlugin, PluginGroup},
};
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;
use player::PlayerPlugin;
use render::RenderPlugins;
use world::WorldPlugins;

fn main() {
    App::new()
        .add_plugins(bevy::DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins(RenderPlugins)
        .add_plugins(WorldPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins((
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian2dPlugin::new(FixedUpdate),
            PhysicsPlugins::new(FixedUpdate),
            #[cfg(debug_assertions)]
            PhysicsDebugPlugin::default(),
        ))
        .run();
}
