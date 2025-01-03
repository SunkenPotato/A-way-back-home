use avian2d::prelude::{Collider, RigidBody};
use bevy::{
    app::{Plugin, Update},
    input::ButtonInput,
    math::{Dir3, Vec3},
    prelude::{
        Bundle, Camera2d, Component, IntoSystemConfigs, KeyCode, Query, Res, Resource, Transform,
        With, Without,
    },
    sprite::Sprite,
    utils::default,
};
use bevy_ecs_ldtk::{
    app::{LdtkEntity, LdtkEntityAppExt},
    GridCoords,
};
use bevy_tnua::{
    prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController},
    TnuaAnimatingState,
};

use crate::{components::EntityDirection, render::animation::AnimationConfig};

static PLAYER_ID: &'static str = "Player";

const PLAYER_DIM: (f32, f32) = (16., 28.);
const MOVEMENT_FACTOR: f32 = 50.;
const SPRINT_FACTOR: f32 = 3.;
const FLOAT_HEIGHT: f32 = (PLAYER_DIM.1 / 2.) + 0.3;
const ACCELERATION: f32 = 50.;
const JUMP_HEIGHT: f32 = 4.;
const RUNNING_MIN: f32 = 80.;
const WALKING_MIN: f32 = 0.1;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (move_player, animate_player, camera_follow_player).chain(),
        )
        .init_resource::<PlayerAnimationPresets>()
        .register_ldtk_entity::<PlayerBundle>(&PLAYER_ID);
    }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
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

#[derive(Debug, PartialEq, Eq)]
enum PlayerState {
    Idle,
    Walking,
    Running,
}

#[derive(Resource)]
struct PlayerAnimationPresets {
    idle: AnimationConfig,
    walk: AnimationConfig,
    run: AnimationConfig,
}

impl Default for PlayerAnimationPresets {
    fn default() -> Self {
        Self {
            idle: AnimationConfig::new(0, 7, 2, None),
            walk: AnimationConfig::new(16, 19, 7, None),
            run: AnimationConfig::new(24, 31, 9, None),
        }
    }
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

fn move_player(
    mut controller: Query<(&mut TnuaController, &mut EntityDirection), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut controller, mut direction)) = controller.get_single_mut() else {
        return;
    };

    let mut direction_v = Vec3::splat(0.);

    if keyboard.pressed(KeyCode::KeyD) {
        direction_v.x = 1.;
        *direction = EntityDirection::R;
    } else if keyboard.pressed(KeyCode::KeyA) {
        direction_v.x = -1.;
        *direction = EntityDirection::L;
    }

    if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
        direction_v *= SPRINT_FACTOR;
    }

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction_v * MOVEMENT_FACTOR,
        desired_forward: Dir3::new(direction_v).ok(),
        float_height: FLOAT_HEIGHT,
        acceleration: ACCELERATION,
        ..default()
    });

    if !keyboard.pressed(KeyCode::Space) {
        return;
    }

    controller.action(TnuaBuiltinJump {
        height: JUMP_HEIGHT,
        ..default()
    });
}

fn animate_player(
    mut query: Query<(
        &mut TnuaAnimatingState<PlayerState>,
        &mut AnimationConfig,
        &TnuaController,
    )>,
    animation_presets: Res<PlayerAnimationPresets>,
) {
    for (mut animating_state, mut config, controller) in &mut query {
        match animating_state.update_by_discriminant({
            let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
                continue;
            };

            let speed = basis_state.running_velocity.length();
            if speed < WALKING_MIN {
                PlayerState::Idle
            } else if speed < RUNNING_MIN {
                PlayerState::Walking
            } else {
                PlayerState::Running
            }
        }) {
            bevy_tnua::TnuaAnimatingStateDirective::Alter {
                old_state: _,
                state,
            } => match state {
                PlayerState::Idle => {
                    *config = animation_presets.idle.clone();
                    config.animation_changed = true;
                }
                PlayerState::Walking => {
                    *config = animation_presets.walk.clone();
                    config.animation_changed = true
                }
                PlayerState::Running => {
                    *config = animation_presets.run.clone();
                    config.animation_changed = true;
                }
            },
            bevy_tnua::TnuaAnimatingStateDirective::Maintain { state: _ } => (),
        };
    }
}

fn camera_follow_player(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };
    let Ok(player) = player.get_single() else {
        return;
    };

    camera.translation = player.translation;
}
