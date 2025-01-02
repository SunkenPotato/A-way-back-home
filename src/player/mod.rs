use avian2d::prelude::{Collider, RigidBody};
use bevy::{
    app::{Plugin, Startup, Update},
    input::ButtonInput,
    log::info,
    math::{Dir3, Vec3},
    prelude::{
        Bundle, Camera2d, Component, IntoSystemConfigs, KeyCode, Query, Res, Single, Transform,
        With, Without,
    },
    sprite::Sprite,
    utils::default,
};
use bevy_ecs_ldtk::{
    app::{LdtkEntity, LdtkEntityAppExt},
    GridCoords,
};
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController};

use crate::utils::spawn_default;

static PLAYER_ID: &'static str = "Player";

const PLAYER_DIM: (f32, f32) = (16., 28.);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_default::<Player>)
            .add_systems(Update, (move_player, camera_follow_player).chain())
            .register_ldtk_entity::<PlayerBundle>(&PLAYER_ID);
    }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    grid_coords: GridCoords,
    tnua_controller: TnuaController,
    collider: Collider,
    rigid_body: RigidBody,
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        entity_instance: &bevy_ecs_ldtk::EntityInstance,
        layer_instance: &bevy_ecs_ldtk::prelude::LayerInstance,
        tileset: Option<&bevy::prelude::Handle<bevy::prelude::Image>>,
        tileset_definition: Option<&bevy_ecs_ldtk::prelude::TilesetDefinition>,
        _asset_server: &bevy::prelude::AssetServer,
        texture_atlases: &mut bevy::prelude::Assets<bevy::prelude::TextureAtlasLayout>,
    ) -> Self {
        Self {
            sprite: bevy_ecs_ldtk::utils::sprite_sheet_from_entity_info(
                entity_instance,
                tileset,
                tileset_definition,
                texture_atlases,
                true,
            ),
            grid_coords: bevy_ecs_ldtk::prelude::GridCoords::from_entity_info(
                entity_instance,
                layer_instance,
            ),
            collider: Collider::rectangle(PLAYER_DIM.0, PLAYER_DIM.1),
            rigid_body: RigidBody::Dynamic,
            ..default()
        }
    }
}

fn move_player(
    mut controller: Query<(&mut TnuaController), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut controller) = controller.get_single_mut() else {
        info!("not running mf");
        return;
    };

    let mut direction = Vec3::splat(0.);

    if keyboard.pressed(KeyCode::KeyD) {
        direction.x = 1.;
    } else if keyboard.pressed(KeyCode::KeyA) {
        direction.x = -1.;
    }

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction * 100.,
        desired_forward: Dir3::new(direction).ok(),
        float_height: PLAYER_DIM.1 / 2. + f32::EPSILON,
        ..default()
    });

    if !keyboard.pressed(KeyCode::Space) {
        return;
    }

    controller.action(TnuaBuiltinJump {
        height: 4.,
        ..default()
    });
}

fn camera_follow_player(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    camera.translation = player.translation;
}
