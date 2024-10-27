use core::f32;
use std::fmt::Display;

use avian2d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::{
    app::{Plugin, Update},
    input::ButtonInput,
    log::info,
    math::{IVec2, Vec3},
    prelude::{
        Bundle, Changed, Commands, Component, Entity, Event, EventReader, EventWriter,
        IntoSystemConfigs, KeyCode, Query, Res, Resource, Transform, With,
    },
    sprite::TextureAtlas,
    time::Time,
};
use bevy_ecs_ldtk::{
    app::{LdtkEntity, LdtkEntityAppExt},
    GridCoords, LdtkSpriteSheetBundle,
};

use crate::{
    components::component::{AnimationConfig, Health, MovementMultiplier},
    identifier,
    util::GridCoordConst,
    world,
};

pub struct PlayerPlugin;

pub const AIR_ACCELERATION: f32 = 700.;
pub const ACCELERATION: f32 = 1600.;
pub const PLAYER_SIZE: (f32, f32) = (16., 19.);

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
pub struct Player {
    direction: Direction,
}

#[derive(Component, Default, PartialEq, Eq)]
pub enum Direction {
    L,
    #[default]
    R,
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
    locked_axes: LockedAxes,
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
            locked_axes: LockedAxes::ROTATION_LOCKED,
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
            (logic_move_controller, visual_move_controller).chain(),
        );
        app.add_systems(Update, (player_void_death, player_death).chain());
        app.add_systems(Update, (exec_animations));

        app.register_ldtk_entity::<PlayerBundle>("Player");
    }
}

fn logic_move_controller(
    mut query: Query<&mut GridCoords, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let movement_direction = if keyboard.just_pressed(KeyCode::Space) {
        GridCoords::Y
    } else if keyboard.just_pressed(KeyCode::KeyA) {
        GridCoords::NEG_X
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        GridCoords::X
    } else {
        GridCoords::ZERO
    };

    for mut coords in &mut query {
        let dest = *coords + movement_direction;
        *coords = dest;
    }
}

fn visual_move_controller(
    mut query: Query<(&mut Transform, &GridCoords), (Changed<GridCoords>, With<Player>)>,
) {
    for (mut transform, grid_coords) in &mut query {
        transform.translation = bevy_ecs_ldtk::utils::grid_coords_to_translation(
            *grid_coords,
            IVec2::from_array([PLAYER_SIZE.0 as i32, PLAYER_SIZE.1 as i32]),
        )
        .extend(transform.translation.z);
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
