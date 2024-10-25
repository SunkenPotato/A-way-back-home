#![allow(unused_parens)]
#![forbid(unsafe_code)]

pub mod render;
pub mod player;
pub mod entity;
pub mod components;
pub mod macros;
pub mod world;

use avian2d::prelude::{Gravity, PhysicsDebugPlugin};
use avian2d::PhysicsPlugins;
use bevy::math::Vec2;
use bevy::{app::App, DefaultPlugins};
use bevy::prelude::{ImagePlugin, PluginGroup};
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian2d::TnuaAvian2dPlugin;
use components::asset::AssetPlugin;
use entity::health::HealthPlugin;

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TnuaAvian2dPlugin::default())
        .add_plugins(TnuaControllerPlugin::default())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(render::camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(world::loader::WorldPlugin)
        .add_plugins(render::sprite::SpritePlugin)
        .add_plugins(AssetPlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(LdtkPlugin)

        .insert_resource(Gravity(Vec2::NEG_Y * 9.81 * 100.));

        if option_env!("DEBUG").is_some() {
            app.add_plugins(PhysicsDebugPlugin::default());
        }

        app.run();
}