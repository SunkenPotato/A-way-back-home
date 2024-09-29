use avian2d::prelude::{ColliderConstructor, Restitution, RigidBody};
use bevy::{
    app::{Plugin, Update},
    asset::AssetServer,
    input::ButtonInput,
    math::{Quat, Vec2, Vec3},
    prelude::{Bundle, IntoSystemConfigs, KeyCode, Query, Res, Transform, With},
    sprite::SpriteBundle,
    utils::default,
};

use crate::components::{Direction, Grounded, Health, Moveable, Player, Speed, Velocity};

pub const DEFAULT_PLAYER_SPRITE: &'static str = "sprites/player/claire-left.png";
pub const PLAYER_SCALE: f32 = 4.;
pub const PLAYER_SIZE: (f32, f32) = (16., 19.);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (Self::movement_system, Self::grounded_rotation_system).chain(),
        );
    }
}

impl PlayerPlugin {
    pub fn movement_system(
        mut query: Query<(&mut Velocity, &Grounded, &mut Transform), With<Player>>,
        kb_input: Res<ButtonInput<KeyCode>>,
    ) {
        for (mut player, grounded, mut transform) in &mut query {
            player.0.x = if kb_input.pressed(KeyCode::KeyD) {
                1.
            } else if kb_input.pressed(KeyCode::KeyA) {
                -1.
            } else {
                0.
            };

            if kb_input.pressed(KeyCode::Space) && grounded.0 {
                player.0.y = 1.
            } else {
                player.0.y = 0.
            }

            if kb_input.pressed(KeyCode::KeyR) && !grounded.0 {
                transform.rotation.z = 0.;
                transform.translation.y += 0.3;
            }
        }
    }

    pub fn construct_default_player(
        asset_server: &Res<AssetServer>,
        x: f32,
        y: f32,
    ) -> impl Bundle {
        (
            SpriteBundle {
                texture: asset_server.load(DEFAULT_PLAYER_SPRITE),
                transform: Transform::from_scale(Vec3::splat(PLAYER_SCALE))
                    .with_translation(Vec3 { x, y, z: 0. }),
                ..default()
            },
            Player::default(),
            Speed::default(2.0),
            Health(20.),
            Direction::L,
            ColliderConstructor::Rectangle {
                x_length: PLAYER_SIZE.0,
                y_length: PLAYER_SIZE.1,
            },
            RigidBody::Dynamic,
            Velocity(Vec2::splat(0.)),
            Moveable,
            Grounded(true),
            Restitution::PERFECTLY_INELASTIC,
        )
    }

    pub fn grounded_rotation_system(mut query: Query<(&Transform, &mut Grounded), With<Moveable>>) {
        for (transform, mut grounded) in &mut query {
            let yaw_deg = Self::quaternion_to_euler(transform.rotation);

            grounded.0 = crate::ternary!(yaw_deg > 5. + f32::EPSILON || yaw_deg == 180. + f32::EPSILON; false, true);
        }
    }

    fn quaternion_to_euler(quat: Quat) -> f32 {
        let (x, y, z, w) = (quat.x, quat.y, quat.z, quat.w);

        let sin_yaw = 2.0 * (w * z + x * y);
        let cos_yaw = 1.0 - 2.0 * (y * y + z * z);
        let yaw = sin_yaw.atan2(cos_yaw);

        let yaw_deg = yaw.to_degrees();

        yaw_deg
    }
}