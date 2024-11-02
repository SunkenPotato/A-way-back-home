use core::f32;
use std::fmt::Display;

use avian2d::prelude::{Collider, RigidBody};
use bevy::{
    app::{Plugin, Update},
    input::ButtonInput,
    log::info,
    math::{IVec2, Vec3},
    prelude::{
        Bundle, Changed, Commands, Component, Entity, Event, EventReader, EventWriter,
        IntoSystemConfigs, KeyCode, Query, Res, Resource, Transform, With,
    },
    sprite::{Sprite, TextureAtlas},
    time::Time,
    utils::default,
};
use bevy_ecs_ldtk::{
    app::{LdtkEntity, LdtkEntityAppExt},
    GridCoords, LdtkSpriteSheetBundle,
};
use bevy_tnua::prelude::{TnuaBuiltinWalk, TnuaController, TnuaControllerBundle};

use crate::{
    components::component::{AnimationConfig, Health, MovementMultiplier},
    identifier, util, world,
};

pub struct PlayerPlugin;

pub const PLAYER_SIZE: (f32, f32) = (16., 19.);
const PLAYER_SIZE_IVEC: IVec2 = IVec2 {
    x: PLAYER_SIZE.0 as i32,
    y: PLAYER_SIZE.1 as i32,
};

#[allow(dead_code)]
const JUMP_KEYS: [KeyCode; 3] = [KeyCode::KeyW, KeyCode::Space, KeyCode::ArrowUp];
const MOVE_LEFT_KEYS: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
const MOVE_RIGHT_KEYS: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];

const AIR_ACCELERATION: f32 = 700.;
const ACCELERATION: f32 = 1600.;

identifier!(PLAYER_STILL, "entity.player.still");

pub enum AnimationState {
    Standing,
    Moving(f32),
    Jumping,
}

#[derive(Resource)]
#[allow(unused)]
pub struct PlayerResource {
    pub size_x: f32,
    pub size_y: f32,
    pub scale: Vec3,
    pub scale_f32: f32,
}

impl Default for PlayerResource {
    fn default() -> Self {
        Self {
            size_x: 16.,
            size_y: 19.,
            scale: world::loader::GLOBAL_SCALE,
            scale_f32: 4.5,
        }
    }
}

#[derive(Component, Default)]
#[allow(unused)]
pub struct Player;

#[derive(Component, Default, PartialEq, Eq)]
pub enum Direction {
    L,
    #[default]
    R,
}

impl Direction {
    /// Returns whether the sprite should be flipped, reliant on the basis that the sprites are originally drawn facing right.
    fn should_flip_sprite(&self) -> bool {
        match self {
            Direction::R => false,
            _ => false,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let final_str = match self {
            Direction::L => "Left",
            Direction::R => "Right",
        };

        write!(f, "{}", final_str)
    }
}

#[derive(Event)]
pub struct PlayerDeath {
    e: Entity,
    cause: PlayerDeathCause,
}

#[allow(dead_code)]
pub enum PlayerDeathCause {
    Void,
    Unknown,
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    grid_coords: GridCoords,
    movement_multiplier: MovementMultiplier,
    animation_config: AnimationConfig,
    collider: Collider,
    rigid_body: RigidBody,
    controller: TnuaControllerBundle,
    direction: Direction,
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
            sprite_sheet_bundle: bevy_ecs_ldtk::utils::sprite_sheet_bundle_from_entity_info(
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
            collider: Collider::rectangle(PLAYER_SIZE.0, PLAYER_SIZE.1),
            player: Player::default(),
            movement_multiplier: MovementMultiplier::default(),
            animation_config: AnimationConfig::new(0, 2, 18),
            rigid_body: RigidBody::Dynamic,
            controller: TnuaControllerBundle::default(),
            direction: Direction::default(),
        }
    }
}

impl Display for PlayerDeathCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_str = match self {
            PlayerDeathCause::Void => "Void",
            PlayerDeathCause::Unknown => "Unknown cause",
        };

        write!(f, "Cause: {}", self_str)
    }
}

#[derive(Event, Default)]
pub struct SpawnPlayerEvent;

#[derive(Resource, Default)]
pub struct PlayerInitalSpawn;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<PlayerResource>();
        app.init_resource::<PlayerInitalSpawn>();

        app.add_event::<SpawnPlayerEvent>();
        app.add_event::<PlayerDeath>();

        app.add_systems(
            Update,
            (
                logic_move_controller,
                visual_move_controller,
                flip_sprite_direction,
            )
                .chain(),
        );

        app.add_systems(Update, (player_void_death, player_death).chain());
        app.add_systems(Update, (exec_animations));

        app.register_ldtk_entity::<PlayerBundle>("Player");
    }
}

fn visual_move_controller(
    mut query: Query<(&mut TnuaController, &mut Direction), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut controller, mut direction)) = query.get_single_mut() else {
        return;
    };

    let mut move_direction = Vec3::ZERO;

    if keyboard_input.any_pressed(MOVE_LEFT_KEYS) {
        move_direction += Vec3::NEG_X;
        *direction = Direction::L;
        bevy::log::debug!(
            "Left key pressed, values: \n\tmd: {move_direction}\n\td:{}",
            *direction
        )
    } else if keyboard_input.any_pressed(MOVE_RIGHT_KEYS) {
        bevy::log::debug!(
            "Right key pressed, values: \n\tmd: {move_direction}\n\td: {}",
            *direction
        );
        move_direction += Vec3::X;
        *direction = Direction::R;
    }

    move_direction *= 10.;

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: move_direction,
        float_height: PLAYER_SIZE.1,
        acceleration: ACCELERATION,
        air_acceleration: AIR_ACCELERATION,
        ..default()
    });
}

fn logic_move_controller(mut query: Query<(&mut GridCoords, &Transform), Changed<GridCoords>>) {
    for (mut grid_coords, transform) in &mut query {
        *grid_coords =
            util::convert::grid_coords_from_vec3(transform.translation, PLAYER_SIZE_IVEC);
    }
}

// Possibly move this to entity.rs
fn flip_sprite_direction(mut query: Query<(&Direction, &mut Sprite), Changed<Direction>>) {
    for (direction, mut sprite) in &mut query {
        sprite.flip_x = direction.should_flip_sprite();
    }
}

fn player_void_death(
    mut query: Query<(&Transform, &mut Health, Entity), With<Player>>,
    mut pdevent: EventWriter<PlayerDeath>,
) {
    for (transform, mut health, entity) in &mut query {
        if transform.translation.y <= -200. {
            health.current = 0.;
            pdevent.send(PlayerDeath {
                e: entity,
                cause: PlayerDeathCause::Void,
            });
        }
    }
}

fn player_death(
    mut commands: Commands,
    mut pdevent: EventReader<PlayerDeath>,
    mut spawn_player: EventWriter<SpawnPlayerEvent>,
) {
    for e in pdevent.read() {
        commands.entity(e.e).despawn();
        spawn_player.send(SpawnPlayerEvent);
        info!("Player died. {}", e.cause);
    }
}

fn exec_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>) {
    for (mut config, mut atlas) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            match atlas.index == config.last_sprite {
                true => atlas.index = config.first_sprite,
                _ => {
                    atlas.index += 1;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}
