use std::time::Duration;

use bevy::{
    app::{FixedUpdate, Plugin},
    prelude::{Component, Query, Res},
    sprite::Sprite,
    time::{Time, Timer},
};

#[derive(Default)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedUpdate, execute_animations);
    }
}

#[derive(Component, Clone)]
pub struct AnimationConfig {
    pub first: usize,
    pub last: usize,
    pub fps: u16,
    pub frame_timer: Timer,
    pub animation_changed: bool,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u16, frame_timer: Option<Timer>) -> Self {
        Self {
            first,
            last,
            fps,
            frame_timer: frame_timer.unwrap_or_else(|| Self::timer_from_fps(fps)),
            animation_changed: false,
        }
    }

    pub fn timer_from_fps(fps: u16) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1. / (fps as f32)),
            bevy::time::TimerMode::Once,
        )
    }

    pub fn reset_timer(&mut self) {
        self.frame_timer = Self::timer_from_fps(self.fps);
    }
}

fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if !config.frame_timer.just_finished() {
            continue;
        }

        let Some(atlas) = &mut sprite.texture_atlas else {
            continue;
        };

        if atlas.index == config.last
            || (config.animation_changed && !(config.first..config.last).contains(&atlas.index))
        {
            atlas.index = config.first;
            config.animation_changed = false;
        } else {
            atlas.index += 1;
        }

        config.reset_timer();
    }
}
