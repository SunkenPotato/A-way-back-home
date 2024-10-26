#![allow(unused_parens)]
#![forbid(unsafe_code)]

pub mod components;
pub mod entity;
pub mod macros;
pub mod player;
pub mod render;
pub mod util;
pub mod world;

use avian2d::prelude::Gravity;
use avian2d::PhysicsPlugins;
use bevy::math::Vec2;
use bevy::prelude::{ImagePlugin, PluginGroup};
use bevy::{app::App, DefaultPlugins};
use bevy_ecs_ldtk::LdtkPlugin;
use components::asset::AssetPlugin;
use entity::health::HealthPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(render::camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(world::loader::WorldPlugin)
        .add_plugins(render::sprite::SpritePlugin)
        .add_plugins(AssetPlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(LdtkPlugin)
        .insert_resource(Gravity(Vec2::NEG_Y * 9.81 * 100.));

    util::debug_mode(&mut app);

    app.run();
}
