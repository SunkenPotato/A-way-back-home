pub use component::{Player, PLAYER_DIM};
pub use movement::SyncCameraWithPlayer;

mod component;
mod movement;

use bevy::{
    app::{Plugin, Update},
    prelude::{IntoSystemConfigs, Resource},
};
use bevy_ecs_ldtk::app::LdtkEntityAppExt;
use component::PlayerBundle;
use movement::{animate_player, camera_follow_player, move_player, sync_camera_with_player};

use crate::render::animation::AnimationConfig;

static PLAYER_ID: &'static str = "Player";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                sync_camera_with_player,
                (move_player, animate_player, camera_follow_player).chain(),
            ),
        )
        .add_event::<SyncCameraWithPlayer>()
        .init_resource::<PlayerAnimationPresets>()
        .register_ldtk_entity::<PlayerBundle>(&PLAYER_ID);
    }
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
