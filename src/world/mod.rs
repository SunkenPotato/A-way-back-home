pub mod level_settings;

use std::sync::LazyLock;

use avian2d::prelude::{Collider, Gravity, RigidBody};
use bevy::{
    app::{plugin_group, Plugin, Startup, Update},
    asset::AssetServer,
    math::Vec2,
    prelude::{
        Bundle, Commands, Component, Event, EventReader, EventWriter, IntoSystemConfigs, Res,
        ResMut,
    },
    utils::default,
};
use bevy_ecs_ldtk::{
    app::{LdtkIntCell, LdtkIntCellAppExt},
    LdtkWorldBundle, LevelSelection,
};
use level_settings::{update_level_settings, LevelSettings};

use crate::{impl_intcell, player::SyncCameraWithPlayer};

static WORLD_PATH: &'static str = "world.ldtk";

pub const GRAVITY: Gravity = Gravity(Vec2::new(0., -98.1));

// TODO: remove
static FIRST_LEVEL: LazyLock<LevelSelection> =
    LazyLock::new(|| LevelSelection::iid("977043b0-c210-11ef-82b1-d58aa0d63a63"));

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

fn initial_level_change(mut commands: Commands) {
    commands.insert_resource(FIRST_LEVEL.clone());
    commands.send_event(ChangeLevel(FIRST_LEVEL.clone()));
}

fn spawn_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load(WORLD_PATH).into(),
        ..default()
    });
}

#[derive(Event)]
pub struct ChangeLevel(pub LevelSelection);

fn change_level(
    mut event_reader: EventReader<ChangeLevel>,
    mut level_selection: ResMut<LevelSelection>,
    mut event_writer: EventWriter<SyncCameraWithPlayer>,
) {
    let Some(last) = event_reader.read().last() else {
        return;
    };

    *level_selection = last.0.clone();
    event_writer.send_default();
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
