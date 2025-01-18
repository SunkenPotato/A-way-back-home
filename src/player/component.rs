use avian2d::prelude::{Collider, RigidBody};
use bevy::{
    prelude::{Bundle, Component},
    sprite::Sprite,
    utils::default,
};
use bevy_ecs_ldtk::{app::LdtkEntity, GridCoords};
use bevy_tnua::{prelude::TnuaController, TnuaAnimatingState};

use crate::{components::EntityDirection, render::animation::AnimationConfig};

pub const PLAYER_DIM: (f32, f32) = (16., 28.);

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub(super) struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    grid_coords: GridCoords,
    tnua_controller: TnuaController,
    collider: Collider,
    rigid_body: RigidBody,
    animation_config: AnimationConfig,
    animating_state: TnuaAnimatingState<PlayerState>,
    direction: EntityDirection,
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
            animation_config: AnimationConfig::new(0, 7, 2, None),
            player: default(),
            animating_state: default(),
            tnua_controller: default(),
            direction: default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum PlayerState {
    Idle,
    Walking,
    Running,
}
