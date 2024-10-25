use core::f32;
use std::fmt::Display;

use avian2d::prelude::{Collider, RigidBody};
use bevy::{app::{Plugin, Startup, Update}, asset::{AssetServer, Assets}, input::ButtonInput, log::{info, warn}, math::{Quat, UVec2, Vec2, Vec3}, prelude::{Commands, Component, Entity, Event, EventReader, EventWriter, IntoSystemConfigs, KeyCode, Query, Res, ResMut, Resource, Transform, With}, sprite::{Sprite, SpriteBundle, TextureAtlas, TextureAtlasLayout}, time::Time, utils::default};
use bevy_tnua::{prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController, TnuaControllerBundle}, TnuaAnimatingState};

use crate::{components::{asset::IndexAsset, component::{AnimationConfig, Health, MovementMultiplier, Velocity}}, identifier, render::sprite::{SPILoaded, SpriteIndexResource}, world};

pub struct PlayerPlugin;

pub const AIR_ACCELERATION: f32 = 700.;
pub const ACCELERATION: f32 = 1600.;

identifier!(PLAYER_STILL, "entity.player.still");

#[derive(Resource)]
#[allow(unused)]
pub struct PlayerResource {
    pub size_x: f32,
    pub size_y: f32,
    pub scale: Vec3,
    pub scale_f32: f32
}

pub enum AnimationState {
    Standing,
    Moving(f32),
    Jumping
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
pub struct PlayerInitalSpawn(bool);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<PlayerResource>();
        app.init_resource::<PlayerInitalSpawn>();
        
        app.add_event::<SpawnPlayerEvent>();
        app.add_event::<PlayerDeath>();

        app.add_systems(Startup, Self::send_spawn_evt);
        app.add_systems(Update, Self::spawn_player);
        app.add_systems(Update, (Self::move_controller).chain());
        app.add_systems(Update, (Self::player_void_death, Self::player_death).chain());
        app.add_systems(Update, (Self::exec_animations));

    }
}

impl PlayerPlugin {

    fn send_spawn_evt(mut e: EventWriter<SpawnPlayerEvent>) {
        e.send_default();
    }
    
    fn move_controller(
        mut query: Query<(&mut TnuaController, &mut Sprite, &mut Direction, &MovementMultiplier, &mut AnimationConfig), With<Player>>, 
        keyboard: Res<ButtonInput<KeyCode>>
    ) {
        let Ok((mut controller, mut sprite, mut direction, multiplier, mut a_config)) = query.get_single_mut() else { return; };

        let mut move_direction = Vec3::ZERO;

        let key_a_pressed = keyboard.pressed(KeyCode::KeyA);
        let key_d_pressed = keyboard.pressed(KeyCode::KeyD);

        if key_a_pressed {
            move_direction -= Vec3::X;

            if let Direction::R = direction.as_ref() { 
                *direction = Direction::L;
                sprite.flip_x = true;
            }
        }

        else if key_d_pressed {
            move_direction += Vec3::X;
            
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
    }
    

    fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, 
        r_player: Res<PlayerResource>, 
        spi_assets: Res<Assets<IndexAsset>>, 
        spi: Res<SpriteIndexResource>, 
        spi_event: EventReader<SPILoaded>, 
        mut spawn_player: EventReader<SpawnPlayerEvent>, 
        mut player_init_spawn: ResMut<PlayerInitalSpawn>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
    ) {

        if !player_init_spawn.0 && spi_event.is_empty() {
            return
        }

        for _ in spawn_player.read() {
            let i_a = match spi_assets.get(&**spi) {
                Some(v) => v,
                None => {
                    warn!("SpriteIndex is not loaded yet, aborting.");
                    return
                }
            };

            let handle = asset_server.load(match i_a.get(&PLAYER_STILL.0){
                Some(v) => v,
                None => {
                    warn!("Can't find {} in SpriteIndex.", PLAYER_STILL.0);
                    return;
                }
            });

            let layout = TextureAtlasLayout::from_grid(UVec2::from_array([r_player.size_x as u32, r_player.size_y as u32]), 3, 1, Some(UVec2 { x: 1, y: 1}), None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);

            let animation_cfg = AnimationConfig::new(0, 2, 18);

            commands.spawn((
                SpriteBundle {
                    texture: handle.clone(),
                    transform: Transform::from_xyz(0., 100., 0.).with_scale(Vec3::splat(r_player.scale_f32)).with_rotation(Quat::IDENTITY),
                    ..default()
                },

                Direction::default(),
                Player::default(),
                TnuaControllerBundle::default(),
                TnuaAnimatingState::<AnimationState>::default() ,

                RigidBody::Dynamic,
                Collider::rectangle(r_player.size_x, r_player.size_y),
                Velocity(Vec2::ZERO),

                MovementMultiplier::default(),
                Health::from((20., 20.)),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: animation_cfg.first_sprite
                },
                animation_cfg
            ));

            player_init_spawn.0 = true;
        }

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
}