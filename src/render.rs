pub mod camera {
    use bevy::{
        app::{Plugin, Startup},
        prelude::{Camera2dBundle, Commands, Query},
        window::Window,
    };

    use crate::warn_fn;

    pub struct CameraPlugin;

    impl Plugin for CameraPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_systems(Startup, Self::init_camera);
        }
    }

    impl CameraPlugin {
        /// Initialize the game camera
        fn init_camera(mut commands: Commands, window: Query<&Window>) {
            warn_fn!("TODO: Add dynamical position computation!");
            let Ok(window) = window.get_single() else {
                panic!("Windows have not yet been instantiated!")
            };

            let mut camera = Camera2dBundle::default();
            camera.projection.scale = 1. / 5.;
            camera.transform.translation.x += window.height() / 5.;
            camera.transform.translation.y += 105.;
            commands.spawn(camera);
        }
    }
}

pub mod sprite {
    use std::ops::Deref;

    use bevy::{
        app::{Plugin, PreStartup, Update},
        asset::{AssetServer, Assets, Handle},
        log::{debug, warn},
        prelude::{
            Commands, Entity, Event, EventReader, EventWriter, IntoSystemConfigs, Query, Res,
            Resource, Transform,
        },
        scene::SceneInstanceReady,
        sprite::SpriteBundle,
        utils::default,
    };

    use crate::components::{
        asset::IndexAsset,
        component::{Identifier, WithSprite},
    };

    pub struct SpritePlugin;

    #[derive(Resource)]
    pub struct SpriteIndexResource {
        pub spi_handle: Handle<IndexAsset>,
        pub is_loaded: bool,
    }

    impl Deref for SpriteIndexResource {
        type Target = Handle<IndexAsset>;

        fn deref(&self) -> &Self::Target {
            &self.spi_handle
        }
    }

    #[derive(Event, Debug)]
    pub struct SPILoaded;

    impl Plugin for SpritePlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_event::<SPILoaded>();
            app.add_systems(PreStartup, Self::load_spi);
            app.add_systems(
                Update,
                (Self::set_spi_state_true, Self::apply_sprites).chain(),
            );
        }
    }

    impl SpritePlugin {
        fn load_spi(mut commands: Commands, asset_server: Res<AssetServer>) {
            let handle: Handle<IndexAsset> = asset_server.load("index/sprites.json");
            let spi = SpriteIndexResource {
                spi_handle: handle,
                is_loaded: false,
            };

            commands.insert_resource(spi);
        }

        fn set_spi_state_true(
            mut commands: Commands,
            index_assets: Res<Assets<IndexAsset>>,
            spi_res: Res<SpriteIndexResource>,
            mut index_loaded_event: EventWriter<SPILoaded>,
        ) {
            if spi_res.is_loaded {
                return;
            }

            if index_assets.get(&spi_res.spi_handle).is_some() {
                debug!("SpriteIndex Resource finished loading.");
                commands.insert_resource(SpriteIndexResource {
                    spi_handle: spi_res.spi_handle.clone(),
                    is_loaded: true,
                });

                index_loaded_event.send(SPILoaded);
            };
        }

        fn apply_sprites(
            mut commands: Commands,
            all: Query<(&Identifier, &Transform, Entity), WithSprite>,
            mut index_loaded_event: EventReader<SPILoaded>,
            index_assets: Res<Assets<IndexAsset>>,
            spi_res: Res<SpriteIndexResource>,
            asset_server: Res<AssetServer>,
            mut load_event: EventReader<SceneInstanceReady>,
        ) {
            if all.is_empty() {
                return;
            }

            for _ in load_event.read() {
                for _ in index_loaded_event.read() {
                    let sprite_index = &index_assets
                        .get(&spi_res.spi_handle)
                        .expect("should not be null because of event.")
                        .0;

                    for (ident, transform, entity) in &all {
                        debug!(
                            "Adding sprite for Identifier {:#?} and Entity: {:#?}",
                            ident.0, entity
                        );

                        let path = match sprite_index.get(&ident.0) {
                            Some(v) => v,
                            None => {
                                warn!("Could not find {} in sprite index! This type of tile will not have a texture at runtime!", ident.0);
                                continue;
                            }
                        };
                        let texture = asset_server.load(path);

                        commands.entity(entity).insert(SpriteBundle {
                            texture,
                            transform: *transform,
                            ..default()
                        });
                    }
                }
            }
        }
    }

    pub struct AnimationPlugin;

    #[allow(unused_variables)]
    impl Plugin for AnimationPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {}
    }
}
