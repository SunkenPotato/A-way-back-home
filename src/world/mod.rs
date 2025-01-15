mod level;
pub mod level_settings;

use avian2d::prelude::{Collider, Gravity, RigidBody};
use bevy::{
    app::{plugin_group, Plugin, Startup, Update},
    asset::AssetServer,
    math::Vec2,
    prelude::{Bundle, Commands, Component, IntoSystemConfigs, Res},
    utils::default,
};
use bevy_ecs_ldtk::{
    app::{LdtkIntCell, LdtkIntCellAppExt},
    LdtkWorldBundle,
};
use level::{change_level, initial_level_change, ChangeLevel};
use level_settings::{update_level_settings, LevelSettings};

use crate::impl_intcell;

static WORLD_PATH: &'static str = "world.ldtk";

pub const GRAVITY: Gravity = Gravity(Vec2::new(0., -98.1));

plugin_group! {
    pub struct WorldPlugins {
        :BasePlugin,
    }
}

#[derive(Default)]
struct BasePlugin;

impl Plugin for BasePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GRAVITY)
            .init_resource::<LevelSettings>()
            .add_event::<ChangeLevel>()
            .register_ldtk_int_cell::<GrassTerrainBundle>(GrassTerrainBundle::INTCELL_ID)
            .add_systems(Update, (change_level, update_level_settings).chain())
            .add_systems(Startup, (spawn_world, initial_level_change).chain());
    }
}

fn spawn_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(WORLD_PATH).into(),
        ..default()
    });
}

pub trait IntCell {
    const DIMENSIONS: (f32, f32);
    const INTCELL_ID: i32;
}

#[derive(Component, Default)]
pub struct GrassTerrain;

#[derive(Default, Bundle)]
pub struct GrassTerrainBundle {
    _m: GrassTerrain,
    collider: Collider,
    rigidbody: RigidBody,
}

impl_intcell!(GrassTerrainBundle | 1; 16.);

impl LdtkIntCell for GrassTerrainBundle {
    fn bundle_int_cell(
        _int_grid_cell: bevy_ecs_ldtk::IntGridCell,
        _layer_instance: &bevy_ecs_ldtk::prelude::LayerInstance,
    ) -> Self {
        Self {
            collider: Collider::rectangle(Self::DIMENSIONS.0, Self::DIMENSIONS.1),
            rigidbody: RigidBody::Static,
            ..default()
        }
    }
}
