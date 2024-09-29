// TODO:
// Create some test terrain, with moving platforms, possibly
// and some air-blocks :)
// TODO, n.2.
// Fix rectangle to extend to edge of screen, i.e., compute window size properly

use avian2d::prelude::ColliderConstructor;
use bevy::{
    app::{Plugin, PreStartup, Startup, Update},
    log::error,
    prelude::{IntoSystemConfigs, Resource, Transform},
};
use scene::{PlayerSpawnMarker, PlayerSpawned};

use crate::components::{
    Direction, Grounded, Moveable, Player, Speed, SpriteMarker, Tile, Velocity,
};

pub struct WorldPlugin;

#[derive(Resource)]
pub struct SavePath {
    old_path: String,
    new_path: String,
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, (scene::load_scene_system).chain())
            .add_systems(Update, (scene::apply_sprites))
            .add_systems(Update, (scene::spawn_player));

        //#[cfg(debug_assertions)]
        {
            error!("Develop detected, adding save_scene_system to schedule");
            app.add_systems(PreStartup, scene::save_scene_system)
        };

        let save_path: SavePath = SavePath {
            old_path: "scenes/test-scene.scn.ron".into(),
            new_path: "scenes/new-test-scene.scn.ron".into(),
        };

        app.register_type::<Player>();
        app.register_type::<Speed>();
        app.register_type::<Direction>();
        app.register_type::<Velocity>();
        app.register_type::<Moveable>();
        app.register_type::<Grounded>();
        app.register_type::<Tile>();
        app.register_type::<SpriteMarker>();
        app.register_type::<Transform>();
        app.register_type::<ColliderConstructor>();
        app.register_type::<PlayerSpawnMarker>();

        app.insert_resource(save_path);
        app.insert_resource(PlayerSpawned(false));
    }
}

pub mod terrain {
    #[allow(unused)]
    pub const TILE_SCALE: f32 = 4.;
    #[allow(unused)]
    pub const TILE_SIZE: (f32, f32) = (16., 16.);
}

pub(super) mod scene {
    use std::{fs::File, io::Write as _};

    use avian2d::prelude::RigidBody;
    use bevy::{
        asset::AssetServer,
        log::info,
        math::{Vec2, Vec3},
        prelude::{
            AppTypeRegistry, Commands, Component, Entity, Query, ReflectComponent, Res, ResMut,
            Resource, Transform, World,
        },
        reflect::Reflect,
        scene::{DynamicScene, DynamicSceneBundle},
        sprite::SpriteBundle,
        tasks::IoTaskPool,
        utils::default,
    };

    use crate::{
        components::{SpriteMarker, Tile},
        ternary,
    };

    use super::SavePath;

    #[derive(Component, Default, Reflect)]
    #[reflect(Component)]
    pub struct PlayerSpawnMarker(Vec2);

    #[derive(Resource)]
    pub struct PlayerSpawned(pub bool);

    pub fn load_scene_system(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        save_path: Res<SavePath>,
    ) {
        info!("LOAD called");
        commands.spawn(DynamicSceneBundle {
            scene: asset_server.load(&save_path.new_path),
            ..default()
        });
    }

    #[cfg(debug_assertions)]
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

    pub fn apply_sprites(
        mut commands: Commands,
        query: Query<(&Transform, &SpriteMarker, Entity)>,
        asset_server: Res<AssetServer>,
    ) {

        for (transform, sprite_marked, entity) in &query {
            let texture = asset_server.load(sprite_marked.0.clone());

            commands.entity(entity).insert(SpriteBundle {
                texture,
                transform: *transform,
                ..default()
            });
        }
    }

    pub fn spawn_player(
        mut commands: Commands,
        query: Query<(&PlayerSpawnMarker)>,
        asset_server: Res<AssetServer>,
        mut player_spawned: ResMut<PlayerSpawned>,
    ) {
        if !player_spawned.0 {
            let mut ctr = 0;
            for spawn_marker in &query {
                commands.spawn(crate::player::PlayerPlugin::construct_default_player(
                    &asset_server,
                    spawn_marker.0.x,
                    spawn_marker.0.y,
                ));
                ctr += 1;
            }

            player_spawned.0 = ternary!(ctr > 0; true, false);
        }
    }
}
