pub fn window_plugin() -> bevy::window::WindowPlugin {
    use bevy::window::{Window, WindowPlugin};

    WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub mod camera {
    use bevy::{
        app::{Plugin, Startup},
        prelude::{Camera2dBundle, Commands, Query},
        window::Window,
    };

    use crate::{error, warn_fn};

    pub struct CameraPlugin;

    impl Plugin for CameraPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_systems(Startup, init_camera);
        }
    }

    /// Initialize the game camera
    fn init_camera(mut commands: Commands, window: Query<&Window>) {
        warn_fn!("TODO: Add dynamical position computation!");
        let Ok(window) = window.get_single() else {
            error::errors::WINDOWS_NOT_INSTANTIATED.exit_with_fatal_error()
        };

        let mut camera = Camera2dBundle::default();
        camera.projection.scale = 1. / 5.;
        camera.transform.translation.x += window.height() / 5.;
        camera.transform.translation.y += 105.;
        commands.spawn(camera);
    }
}

pub mod animate {
    use bevy::{
        app::{Plugin, Update},
        prelude::{Query, Res, With},
        sprite::TextureAtlas,
        time::Time,
    };

    use crate::components::component::{Animatable, AnimationConfig};

    pub struct AnimationPlugin;

    impl Plugin for AnimationPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_systems(Update, execute_animations);
        }
    }

    fn execute_animations(
        time: Res<Time>,
        mut query: Query<(&mut AnimationConfig, &mut TextureAtlas), With<Animatable>>,
    ) {
        for (mut config, mut atlas) in &mut query {
            config.frame_timer.tick(time.delta());

            if config.frame_timer.just_finished() {
                if atlas.index >= config.sprite_indices.last_sprite {
                    atlas.index = config.sprite_indices.first_sprite;
                } else {
                    atlas.index += 1;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}
