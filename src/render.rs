pub mod camera {
    use bevy::{app::{Plugin, Startup}, prelude::{Camera2dBundle, Commands, Transform}, utils::default};

    pub struct CameraPlugin;

    impl Plugin for CameraPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_systems(Startup, Self::init_camera);
        }
    }

    impl CameraPlugin {
        
        /// Initialize the game camera
        fn init_camera(mut commands: Commands) {
            commands.spawn(
                Camera2dBundle {
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..default()
                }
            );
        }
    }
}

pub mod sprite {
    use bevy::{app::Plugin, asset::AssetServer, prelude::{Commands, Entity, Query, Res}};

    use crate::components::component::{Identifier, WithSprite};

    pub struct SpritePlugin;

    impl Plugin for SpritePlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            
        }
    }

    impl SpritePlugin {
        #[allow(unused)]
        fn apply_sprites(mut commands: Commands, all: Query<(&Identifier<'static>, Entity), WithSprite>, asset_server: Res<AssetServer>) {
            for (identifier, entity) in &all {
                todo!()
            }
        }
    }
}