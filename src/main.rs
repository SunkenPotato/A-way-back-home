#![allow(unused_parens)]
#![deny(unsafe_code, reason = "This should be a safe program")]
#![deny(unused_imports, reason = "Don't forget me")]
#![forbid(clippy::panic, reason = "Use FatalErrors instead")]

pub mod components;
pub mod entity;
pub mod error;
pub mod macros;
pub mod player;
pub mod render;
pub mod util;
pub mod world;

use avian2d::prelude::Gravity;
use avian2d::PhysicsPlugins;
use bevy::log::LogPlugin;
use bevy::math::Vec2;
use bevy::prelude::{ImagePlugin, PluginGroup};
use bevy::{app::App, DefaultPlugins};
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;
use entity::health::HealthPlugin;

fn main() {
    let mut app = App::new();

    let mut log_plugin = LogPlugin {
        filter: "wgpu=error,naga=warn,avian2d=error,".to_string(),
        ..Default::default()
    };

    util::debug_mode(&mut app, &mut log_plugin);

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(render::window_plugin())
            .set(log_plugin),
    )
    .add_plugins(PhysicsPlugins::default())
    .add_plugins(TnuaControllerPlugin::default())
    .add_plugins(TnuaAvian2dPlugin::default())
    .add_plugins(render::camera::CameraPlugin)
    .add_plugins(player::PlayerPlugin)
    .add_plugins(world::loader::WorldPlugin)
    .add_plugins(HealthPlugin)
    .add_plugins(LdtkPlugin)
    .add_plugins(render::animate::AnimationPlugin)
    .insert_resource(Gravity(Vec2::NEG_Y * 9.81 * 100.));

    app.run();
}
