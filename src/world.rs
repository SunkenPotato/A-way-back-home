pub mod loader {
    
    use std::{fs::File, io::Write as _, num::NonZero, sync::LazyLock};

    use avian2d::prelude::{ColliderConstructor, RigidBody};
    use bevy::{app::{App, Plugin, PreStartup, Startup}, asset::{AssetServer, Handle}, log::error, math::Vec3, prelude::{AppTypeRegistry, Commands, Res, Resource, Transform, World}, scene::{DynamicScene, DynamicSceneBundle}, tasks::IoTaskPool, utils::default};

    use crate::components::component::{Identifier, SpriteMarker, Tile, Velocity};

    use super::tile;

    pub struct WorldPlugin;

    pub static EXIT_ERROR_CODE: LazyLock<NonZero<u8>> = LazyLock::new(|| NonZero::new(1).unwrap());

    pub const GLOBAL_SCALE: Vec3 = Vec3::from_slice(&[4.5, 4.5, 0.]);

    
    #[derive(Resource, Clone)]
    pub struct SavePath(String);

    #[derive(Resource, Debug)]
    pub struct DSHandle(pub Handle<DynamicScene>);

    impl Plugin for WorldPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            add_type_reg(app);

            app.insert_resource(SavePath("scenes/test-scene.scn.ron".into()));

            app.add_systems(Startup, Self::load_world);

            if let Some(_) = option_env!("DEV_MODE") {
                app.add_systems(PreStartup, Self::save_world);
            };
        }
    }

    fn add_type_reg(app: &mut App) -> &mut App {
        app
            .register_type::<Tile>()
            .register_type::<Identifier>()
            .register_type::<Velocity>()
            .register_type::<SpriteMarker>()

            .register_type::<ColliderConstructor>();

        app
    }

    impl WorldPlugin {
        fn load_world(mut commands: Commands, asset_server: Res<AssetServer>, sp: Res<SavePath>) {
            let ds_handle = asset_server.load(sp.0.clone());

            commands.insert_resource(DSHandle(ds_handle.clone()));
            
            commands.spawn(DynamicSceneBundle {
                scene: ds_handle,
                ..default()
            });

        }

        fn save_world(world: &mut World) {

            let save_path = match world.get_resource::<SavePath>().cloned() {
                Some(v) => v,
                None => {
                    error!("Scene path is unexpectedly None");
                    world.send_event(bevy::app::AppExit::Error(*EXIT_ERROR_CODE));
                    return;
                }
            };

            let mut scn_world = World::new();

            let type_reg = world.resource::<AppTypeRegistry>().clone();
            scn_world.insert_resource(type_reg);
            

            /* SPAWN ELEMENTS HERE */
            // BEGIN
            for x in 0..10 {
                let x_pos = (x as f32 * super::tile::TILE_SIZE.0 * super::tile::TILE_SCALE.x);

                scn_world.spawn((
                    Tile,
                    tile::idents::DIRT.clone(),
                    ColliderConstructor::Rectangle { x_length: super::tile::TILE_SIZE.0, y_length: super::tile::TILE_SIZE.1 },
                    RigidBody::Static,
                    Transform::from_xyz(x_pos, -64., 0.)
                        .with_scale(super::tile::TILE_SCALE),
                    SpriteMarker
                ));
            }
            // END

            /* INSERT RESOURCES HERE */
            // BEGIN
            
            // END

            let scn = DynamicScene::from_world(&scn_world);
            let type_reg = world.resource::<AppTypeRegistry>().read();
            let serialized_scn = scn.serialize(&type_reg).unwrap();

            IoTaskPool::get()
                .spawn(async move {
                    File::create(format!("assets/{}", save_path.0))
                        .and_then(|mut file| file.write(serialized_scn.as_bytes()))
                        .expect("Error writing scene to file")
                })
                .detach();

        }

        /*
        pub fn save_scene_system(world: &mut World) {
            use avian2d::prelude::{Collider, ColliderConstructor};
    
            use crate::world::terrain::TILE_SIZE;
    
            info!("SAVE called");
            let save_path = world.get_resource::<SavePath>().unwrap();
            let path = save_path.new_path.clone();
    
            let mut scene_world = World::new();
    
            let type_registry = world.resource::<AppTypeRegistry>().clone();
            scene_world.insert_resource(type_registry);
    
            for x in 0..10 {
                let x_pos = (x as f32 * super::terrain::TILE_SIZE.0 * super::terrain::TILE_SCALE);
    
                scene_world.spawn((
                    Tile {
                        pos: Vec2 { x: x_pos, y: -32. },
                        size: Vec2 {
                            x: TILE_SIZE.0,
                            y: TILE_SIZE.1,
                        },
                    },
                    ColliderConstructor::Rectangle {
                        x_length: TILE_SIZE.0,
                        y_length: TILE_SIZE.1,
                    },
                    RigidBody::Static,
                    SpriteMarker("sprites/terrain/dirt-1.png".into()),
                    Transform::from_xyz(x_pos, -32., 0.)
                        .with_scale(Vec3::splat(super::terrain::TILE_SCALE)),
                ));
            }
    
            for y in 0..5 {
                let y_pos = (y as f32 * super::terrain::TILE_SIZE.1 * super::terrain::TILE_SCALE) + 32.;
    
                scene_world.spawn((
                    Tile {
                        pos: Vec2 {
                            x: 128.,
                            y: y_pos as f32,
                        },
                        size: Vec2 {
                            x: TILE_SIZE.0,
                            y: TILE_SIZE.1,
                        },
                    },
                    ColliderConstructor::Rectangle {
                        x_length: TILE_SIZE.0,
                        y_length: TILE_SIZE.1,
                    },
                    RigidBody::Static,
                    SpriteMarker("sprites/terrain/dirt-1.png".into()),
                    Transform::from_xyz(32., y_pos, 0.)
                        .with_scale(Vec3::splat(super::terrain::TILE_SCALE)),
                ));
            }
    
            scene_world.spawn(PlayerSpawnMarker(Vec2 { x: 0., y: 60. }));
    
            let scene = DynamicScene::from_world(&scene_world);
    
            let type_reg = world.resource::<AppTypeRegistry>().read();
    
            let serialized_scene = scene.serialize(&type_reg).unwrap();
    
            //info!("{}", serialized_scene);
    
            IoTaskPool::get()
                .spawn(async move {
                    File::create(format!("assets/{path}"))
                        .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                        .expect("Error writing scene to file")
                })
                .detach();
        }
        */
    }
}

pub mod tile {
    use bevy::math::Vec3;

    pub const TILE_SCALE: Vec3 = super::loader::GLOBAL_SCALE;
    pub const TILE_SIZE: (f32, f32) = (16., 16.);

    // Define Tile Identifiers here
    // BEGIN

    /// The format for Identifiers is:
    /// ```
    /// entity.type.name
    /// ```
    /// 
    /// I'm not explaning myself in a private project 
    ///
    pub mod idents {
        use crate::{components::component::Identifier, identifier};
        use std::sync::LazyLock;

        identifier!(DIRT, "tile.ter.dirt");
        identifier!(GRASS, "tile.dec.grass");
    }
    // END
}