use core::f32;
use std::fmt::Display;

use bevy::{app::{Plugin, Update}, input::ButtonInput, log::info, math::Vec3, prelude::{Bundle, Commands, Component, Entity, Event, EventReader, EventWriter, IntoSystemConfigs, KeyCode, Query, Res, Resource, Transform, With}, sprite::{Sprite, TextureAtlas}, time::Time};
use bevy_ecs_ldtk::{app::LdtkEntityAppExt, GridCoords, LdtkEntity, LdtkSpriteSheetBundle};
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController, TnuaControllerBundle};

use crate::{components::component::{AnimationConfig, Health, MovementMultiplier}, identifier, util::GridCoordConst, world};

pub struct PlayerPlugin;

pub const AIR_ACCELERATION: f32 = 700.;
pub const ACCELERATION: f32 = 1600.;

identifier!(PLAYER_STILL, "entity.player.still");

pub enum AnimationState {
    Standing,
    Moving(f32),
    Jumping
}

#[derive(Resource)]
#[allow(unused)]
pub struct PlayerResource {
    pub size_x: f32,
    pub size_y: f32,
    pub scale: Vec3,
    pub scale_f32: f32
}

impl Default for PlayerResource {
    fn default() -> Self {
        Self {
            size_x: 16.,
            size_y: 19.,
            scale: world::loader::GLOBAL_SCALE,
            scale_f32: 4.5
        }
    }
}

#[derive(Component, Default)]
#[allow(unused)]
pub struct Player {
    direction: Direction
}

#[derive(Component, Default, PartialEq, Eq)]
pub enum Direction {
    L, 
    #[default]
    R
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let final_str = match self {
            Direction::L => "Left",
            Direction::R => "Right"
        };

        write!(f, "{}", final_str)
    }
}

#[derive(Event)]
pub struct PlayerDeath {
    e: Entity,
    cause: PlayerDeathCause
}

#[allow(dead_code)]
pub enum PlayerDeathCause {
    Void,
    Unknown
} 

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    controller: TnuaControllerBundle,
    movement_multiplier: MovementMultiplier,
    animation_config: AnimationConfig
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player::default(),
            sprite_sheet_bundle: LdtkSpriteSheetBundle::default(),
            grid_coords: GridCoords::default(),
            controller: TnuaControllerBundle::default(),
            movement_multiplier: MovementMultiplier::default(),
            animation_config: AnimationConfig::new(0, 2, 18)
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
        
        app.add_systems(Update, (logic_move_controller).chain());
        app.add_systems(Update, (player_void_death, player_death).chain());
        app.add_systems(Update, (exec_animations));
        
        app.register_ldtk_entity::<PlayerBundle>("Player");
    }
}

fn logic_move_controller(
    mut query: Query<(&mut TnuaController, &mut Sprite, &mut Direction, &MovementMultiplier, &mut AnimationConfig, &mut GridCoords), With<Player>>,  
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut controller, mut sprite, mut direction, multiplier, mut a_config, mut grid_coords)) = query.get_single_mut() else { return; };
    info!("Found following: ([controller], [sprite], {}, {multiplier}, {}, {grid_coords:#?})", *direction, *a_config);

    let mut move_direction = Vec3::ZERO;
    let mut grid_coords_direct = GridCoords::ZERO;

    let key_a_pressed = keyboard.pressed(KeyCode::KeyA);
    let key_d_pressed = keyboard.pressed(KeyCode::KeyD);

    if key_a_pressed {
        move_direction -= Vec3::X;
        grid_coords_direct = GridCoords::NEG_X;

        if let Direction::R = direction.as_ref() { 
            *direction = Direction::L;
            sprite.flip_x = true;
        }
    }

    else if key_d_pressed {
        move_direction += Vec3::X;
        grid_coords_direct = GridCoords::X;
        
        if let Direction::L = direction.as_ref() {
            *direction = Direction::R;
            sprite.flip_x = false;
        }
    }

    let standing_on_solid = match controller.concrete_basis::<TnuaBuiltinWalk>() {
        Some((_, basis_state)) => basis_state.standing_on_entity().is_some(),
        _ => false
    };
    

    if (key_a_pressed || key_d_pressed) && a_config.frame_timer.finished() && standing_on_solid {
        a_config.frame_timer = AnimationConfig::timer_from_fps(a_config.fps);
    }      

    let final_direction: Vec3 = move_direction * 50. * **multiplier;

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: final_direction,
        float_height: 44. + f32::EPSILON,
        desired_forward: Vec3::X,
        acceleration: ACCELERATION,
        air_acceleration: AIR_ACCELERATION,
        ..Default::default()
    });

    if keyboard.just_pressed(KeyCode::Space) {
        controller.action(TnuaBuiltinJump {
            height: 100. * multiplier.y,
            ..Default::default()
        });
    }

    *grid_coords += grid_coords_direct;
}

fn player_void_death(mut query: Query<(&Transform, &mut Health, Entity), With<Player>>, mut pdevent: EventWriter<PlayerDeath>) {
    for (transform, mut health, entity) in &mut query {
        if transform.translation.y <= -200. {
            health.current = 0.;
            pdevent.send(PlayerDeath { e: entity, cause: PlayerDeathCause::Void });
        }
    }
}

fn player_death(mut commands: Commands, mut pdevent: EventReader<PlayerDeath>, mut spawn_player: EventWriter<SpawnPlayerEvent>) {
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