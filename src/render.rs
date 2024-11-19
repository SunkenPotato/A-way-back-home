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
        app::{Plugin, Startup, Update},
        math::Vec3,
        prelude::*,
        time::Time,
        window::Window,
    };

    use crate::{
        error,
        player::Player,
        world::loader::{BorderWall, CurrentLevel},
    };

    pub struct CameraPlugin;

    const CAMERA_DECAY: f32 = 2.;

    impl Plugin for CameraPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_systems(Startup, init_camera);
            app.add_systems(Update, move_camera);
        }
    }

    /// Initialize the game camera
    fn init_camera(
        mut commands: Commands,
        window: Query<&Window>,
        level: Option<Res<CurrentLevel>>,
    ) {
        let Ok(window) = window.get_single() else {
            error::errors::WINDOWS_NOT_INSTANTIATED.exit_with_fatal_error()
        };

        let mut camera = Camera2dBundle::default();
        camera.projection.scale = 1. / 5.;
        camera.transform.translation.x += window.height() / 5.;
        camera.transform.translation.y += 105.;
        commands.spawn(camera);
    }

    fn move_camera(
        player: Query<(&Transform), (Without<Camera>, Without<BorderWall>, With<Player>)>,
        border_walls: Query<(&Transform), (Without<Player>, With<Camera>, With<BorderWall>)>,
        mut camera: Query<&mut Transform, (Without<Player>, Without<BorderWall>, With<Camera>)>,
        time: Res<Time>,
    ) {
        let Ok(mut camera_transform) = camera.get_single_mut() else {
            return;
        };

        let Ok(player_transform) = player.get_single() else {
            return;
        };

        if border_walls.iter().len() == 0 {
            bevy::log::info!("border walls not fuond???")
        }

        if camera_transform
            .translation
            .distance(player_transform.translation)
            > 20.
        {
            let Vec3 { x, y, .. } = player_transform.translation;
            let direction = Vec3::new(x, y, camera_transform.translation.z);

            camera_transform.translation = camera_transform
                .translation
                .lerp(direction, time.delta_seconds() * CAMERA_DECAY);
        }
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
