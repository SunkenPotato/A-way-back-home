use avian2d::prelude::{Collider, RigidBody};
use bevy::{app::{Plugin, Startup, Update}, asset::{AssetServer, Handle}, input::ButtonInput, log::info, math::{Quat, Vec2, Vec3}, prelude::{Commands, Component, Image, IntoSystemConfigs, KeyCode, Query, Res, Transform, With}, sprite::SpriteBundle, utils::default};
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController, TnuaControllerBundle};

use crate::components::component::Velocity;

pub struct PlayerPlugin;

const PLAYER_SPRITE_PATH: &str = "sprites/player/claire-left.png";
const PLAYER_SCALE: Vec3 = Vec3::from_slice(&[4.5, 4.5, 0.]);
const PLAYER_SIZE: (f32, f32) = (16., 19.);

const DEF_PLAYER_SPEED: f32 = 20.;

#[derive(Component, Default)]
pub struct Player {
    direction: Direction
}

#[derive(Default)]
pub enum Direction {
    L, 
    #[default]
    R
}

#[derive(Component)]
pub struct SpeedMultiplier(Vec3);

impl Default for SpeedMultiplier {
    fn default() -> Self {
        Self(Vec3::splat(10.))
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, Self::spawn_player);
        app.add_systems(Update, (Self::move_controller).chain());
    }
}

impl PlayerPlugin {
    #[warn(unstable_features, reason = "Unfinished")]
    fn move_controller(
        mut query: Query<(&mut TnuaController), With<Player>>, 
        keyboard: Res<ButtonInput<KeyCode>>
    ) {
        let Ok(mut controller) = query.get_single_mut() else {
            info!("ctrl not found");
            return;
        };
    
        let mut direction = Vec3::ZERO;
    
        if keyboard.pressed(KeyCode::ArrowUp) {
            direction -= Vec3::Z;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            direction += Vec3::Z;
        }
        if keyboard.pressed(KeyCode::ArrowLeft) {
            direction -= Vec3::X;
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            direction += Vec3::X;
        }
    
        // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
        // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
        // just fall.
        controller.basis(TnuaBuiltinWalk {
            // The `desired_velocity` determines how the character will move.
            desired_velocity: Vec3::new(2000.0, 0., 0.),
            // The `float_height` must be greater (even if by little) from the distance between the
            // character's center and the lowest point of its collider.
            float_height: 9.,
            // `TnuaBuiltinWalk` has many other fields for customizing the movement - but they have
            // sensible defaults. Refer to the `TnuaBuiltinWalk`'s documentation to learn what they do.
            ..Default::default()
        });
        // Feed the jump action every frame as long as the player holds the jump button. If the player
        // stops holding the jump button, simply stop feeding the action.
        if keyboard.pressed(KeyCode::Space) {
            controller.action(TnuaBuiltinJump {
                // The height is the only mandatory field of the jump button.
                height: 60.0,
                // `TnuaBuiltinJump` also has customization fields with sensible defaults.
                ..Default::default()
            });
        }
    }
    

    fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {

        let handle: Handle<Image> = asset_server.load(PLAYER_SPRITE_PATH);

        commands.spawn((
            SpriteBundle {
                texture: handle,
                transform: Transform::from_xyz(0., 16., 0.).with_scale(Vec3::splat(PLAYER_SCALE.x)).with_rotation(Quat::IDENTITY),
                ..default()
            },
            Player::default(),
            TnuaControllerBundle::default(),
            RigidBody::Dynamic,
            Collider::rectangle(PLAYER_SIZE.0, PLAYER_SIZE.1),
            Velocity(Vec2::ZERO),
            SpeedMultiplier::default()
        ));
    }
}