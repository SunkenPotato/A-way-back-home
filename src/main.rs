#![allow(unused_parens)]
pub mod player;
pub mod components;
pub mod entity;
pub mod world;

use avian2d::{prelude::{Gravity, PhysicsDebugPlugin}, PhysicsPlugins};
use bevy::{app::{App, PluginGroup, Startup, Update}, math::Vec2, prelude::{Camera2dBundle, Commands, ImagePlugin, IntoSystemConfigs, Transform}, utils::default, window::{Window, WindowPlugin}, DefaultPlugins};
use components::GameOver;
use player::PlayerPlugin;

fn main() {
    let default_plugins = DefaultPlugins::
        set(DefaultPlugins, ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoVsync,
                resizable: false,
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        });

    App::new()
        .add_plugins(default_plugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(PlayerPlugin)
        .insert_resource(Gravity(Vec2::NEG_Y * 9.81 * 100.))
        .add_systems(Startup, (setup_system, world::terrain::setup_terrain).chain())
        .add_systems(Update, (
            entity::grounded_system,
            entity::moveable_system
        ))
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));

    commands.insert_resource(GameOver(false));
}