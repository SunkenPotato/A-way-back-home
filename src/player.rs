use core::f32;

use avian2d::prelude::{Collider, RigidBody};
use bevy::{app::{Plugin, Update}, asset::{AssetServer, Assets}, input::ButtonInput, log::warn, math::{Quat, Vec2, Vec3}, prelude::{Commands, Component, EventReader, IntoSystemConfigs, KeyCode, Query, Res, Resource, Transform, With}, sprite::{Sprite, SpriteBundle}, utils::default};
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController, TnuaControllerBundle};

use crate::{components::{asset::IndexAsset, component::{MovementMultiplier, Velocity}}, identifier, render::sprite::{SPILoaded, SpriteIndexResource}};

pub struct PlayerPlugin;

identifier!(PLAYER_STILL, "entity.player.still");

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
            scale: Vec3::from_slice(&[4.5, 4.5, 0.]),
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

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<PlayerResource>();

        app.add_systems(Update, Self::spawn_player);
        app.add_systems(Update, (Self::move_controller).chain());
    }
}

impl PlayerPlugin {
    
    fn move_controller(
        mut query: Query<(&mut TnuaController, &mut Sprite, &mut Direction, &MovementMultiplier), With<Player>>, 
        keyboard: Res<ButtonInput<KeyCode>>
    ) {
        let Ok((mut controller, mut sprite, mut direction, multiplier)) = query.get_single_mut() else { return; };

        let mut move_direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyA) {
            move_direction -= Vec3::X;
            if let Direction::R = direction.as_ref() { 
                *direction = Direction::L;
                sprite.flip_x = true;
            }
        }
        if keyboard.pressed(KeyCode::KeyD) {
            move_direction += Vec3::X;
            if let Direction::L = direction.as_ref() {
                *direction = Direction::R;
                sprite.flip_x = false;
            }
        }

        let fd = move_direction * 50. * **multiplier;

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: fd,
            float_height: 44. + f32::EPSILON,
            desired_forward: Vec3::X,
            //cling_distance: 70.,
            acceleration: 2000.,
            air_acceleration: 1000.,
            ..Default::default()
        });

        if keyboard.just_pressed(KeyCode::Space) {
            controller.action(TnuaBuiltinJump {
                height: 80.0 * multiplier.y,
                ..Default::default()
            });
        }
    }
    

    fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>, r_player: Res<PlayerResource>, spi_assets: Res<Assets<IndexAsset>>, spi: Res<SpriteIndexResource>, mut spi_event: EventReader<SPILoaded>) {
        for _ in spi_event.read_with_id() {
            warn!("e recv");
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
        
            commands.spawn((
                SpriteBundle {
                    texture: handle,
                    transform: Transform::from_xyz(0., 100., 0.).with_scale(Vec3::splat(r_player.scale_f32)).with_rotation(Quat::IDENTITY),
                    ..default()
                },
                Direction::default(),
                Player::default(),
                TnuaControllerBundle::default(),
                RigidBody::Dynamic,
                Collider::rectangle(r_player.size_x, r_player.size_y),
                Velocity(Vec2::ZERO),
                MovementMultiplier::default()
            ));
        }

    }

}