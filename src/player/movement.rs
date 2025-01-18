use bevy::{
    input::ButtonInput,
    log::{error, warn},
    math::{Dir3, Vec3},
    prelude::{
        Camera2d, Event, EventReader, GlobalTransform, KeyCode, Query, Res, Transform, With,
        Without,
    },
    utils::default,
};
use bevy_tnua::{
    prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController},
    TnuaAnimatingState,
};

use crate::{
    components::EntityDirection, render::animation::AnimationConfig,
    world::level_settings::LevelSettings,
};

use super::{
    component::{Player, PlayerState, PLAYER_DIM},
    PlayerAnimationPresets,
};

const MOVEMENT_FACTOR: f32 = 50.;
const SPRINT_FACTOR: f32 = 3.;
const FLOAT_HEIGHT: f32 = (PLAYER_DIM.1 / 2.) + 0.3;
const ACCELERATION: f32 = 50.;
const JUMP_HEIGHT: f32 = 24.;
const RUNNING_MIN: f32 = 80.;
const WALKING_MIN: f32 = 0.1;

pub(super) fn move_player(
    mut controller: Query<(&mut TnuaController, &mut EntityDirection, &Transform), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut controller, mut direction, transform)) = controller.get_single_mut() else {
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

    direction_v *= MOVEMENT_FACTOR;
    direction_v.z = transform.translation.z;

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction_v,
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

pub(super) fn camera_follow_player(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&GlobalTransform, (With<Player>, Without<Camera2d>)>,
    level_settings: Res<LevelSettings>,
) {
    if *level_settings.camera_follow == false {
        // if the level asks us for the camera to stay put, it'll do so.
        return;
    }

    let Ok(mut camera) = camera.get_single_mut() else {
        error!("Camera should exist at this point!");
        return;
    };
    let Ok(player) = player.get_single() else {
        warn!("Expected player, found none.");
        return;
    };

    camera.translation.x = player.translation().x;
    camera.translation.y = player.translation().y;
}

#[derive(Event, Default)]
pub struct SyncCameraWithPlayer;

// For level changes
pub(super) fn sync_camera_with_player(
    mut event_reader: EventReader<SyncCameraWithPlayer>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        error!("Camera instance should exist at this point!");
        return;
    };
    let Ok(player) = player.get_single() else {
        warn!("Expected player, found none.");
        return;
    };

    for _ in event_reader.read() {
        camera.translation.x = player.translation.x;
        camera.translation.y = player.translation.y;
    }
}

pub(super) fn animate_player(
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
