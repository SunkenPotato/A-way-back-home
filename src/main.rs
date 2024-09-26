pub mod render;
pub mod player;
pub mod entity;

use avian2d::{prelude::Gravity, PhysicsPlugins};
use bevy::{app::{App, PluginGroup, Startup, Update}, input::common_conditions::{input_just_pressed, input_pressed}, math::Vec2, prelude::{ImagePlugin, IntoSystemConfigs, KeyCode}, DefaultPlugins};
use render::setup_camera;


fn main() {

    let plugins = DefaultPlugins::set(DefaultPlugins, ImagePlugin::default_nearest());

    App::new()
        .add_plugins(plugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::NEG_Y * 9.81 * 100.0))
        .add_systems(Startup, (setup_camera, render::draw_scene, player::setup).chain())
        .add_systems(Update, (
            (player::move_right).run_if(input_pressed(KeyCode::KeyD)),
            (player::move_left).run_if(input_pressed(KeyCode::KeyA)),
            (player::jump).run_if(input_just_pressed(KeyCode::Space))))
        .add_systems(Update, (entity::gravity_s, entity::collision))
        .run();
}