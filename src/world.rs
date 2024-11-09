pub mod loader {

    use std::fmt::Display;

    use avian2d::prelude::{Collider, RigidBody};
    use bevy::{
        app::{Plugin, Startup, Update},
        asset::{AssetServer, Assets, Handle},
        math::Vec3,
        prelude::{
            Bundle, Commands, Component, Deref, Event, EventReader, EventWriter, Query, Res,
            ResMut, Resource, With,
        },
        utils::default,
    };
    use bevy_ecs_ldtk::{
        app::{LdtkIntCell, LdtkIntCellAppExt},
        assets::{LdtkProject, LevelMetadataAccessor},
        ldtk::{FieldInstance, FieldValue},
        GridCoords, LdtkWorldBundle, LevelIid, LevelSelection,
    };
    use strum_macros::EnumIter;

    use crate::{
        error,
        player::{Player, SpawnPlayerEvent},
    };

    pub struct WorldPlugin;
    pub const GLOBAL_SCALE: Vec3 = Vec3::from_slice(&[3.5, 3.5, 0.]);

    const GRASS_INT_CELL: i32 = 1;

    const SPAWNPOINT_IDENT: &'static str = "Spawnpoint";

    #[derive(Event, Deref)]
    pub struct ChangeLevel(usize);

    impl Plugin for WorldPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_systems(Startup, load_world);
            app.add_systems(Update, set_level);
            app.add_event::<ChangeLevel>();
            app.register_ldtk_int_cell::<GrassTerrainBundle>(GRASS_INT_CELL);

            app.insert_resource(SpawnPoint(GridCoords::new(0, 0)));

            // remove!
            app.add_systems(Update, get_level_spawnpoint);
            app.add_systems(Update, transport_player_to_spawnpoint);
        }
    }

    fn load_world(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut event_writer: EventWriter<ChangeLevel>,
    ) {
        let ldtk_handle = asset_server.load("scenes/test-world.ldtk");

        let ldtk_bundle = LdtkWorldBundle {
            ldtk_handle,
            ..default()
        };

        commands.spawn(ldtk_bundle);
        event_writer.send(ChangeLevel(0));
    }

    // Always use this function instead of directly inserting the resource
    fn set_level(mut commands: Commands, mut change_level_e: EventReader<ChangeLevel>) {
        for event in change_level_e.read() {
            commands.insert_resource(LevelSelection::index(event.0));
        }
    }

    fn get_level_spawnpoint(
        query: Query<&LevelIid>,
        projects: Query<&Handle<LdtkProject>>,
        assets: Res<Assets<LdtkProject>>,
        mut spawnpoint: ResMut<SpawnPoint>,
        mut spawn_player: EventReader<SpawnPlayerEvent>,
    ) {
        for _ in spawn_player.read() {
            for level_iid in &query {
                let only_project = assets.get(projects.single()).expect("a project");

                let level = only_project.get_raw_level_by_iid(level_iid.get()).unwrap();

                let new_sp = match spawnpoint_from_fields(&level.field_instances) {
                    Ok(v) => v,
                    Err(e) => {
                        error::errors::SPAWNPOINT_ERR.trigger_msg(e);
                        return;
                    }
                };

                *spawnpoint = new_sp;
            }
        }
    }

    fn transport_player_to_spawnpoint(
        mut player: Query<&mut GridCoords, With<Player>>,
        spawnpoint: Res<SpawnPoint>,
        mut spawn_player: EventReader<SpawnPlayerEvent>,
    ) {
        for _ in spawn_player.read() {
            for mut grid_coordinates in &mut player {
                *grid_coordinates = **spawnpoint;
            }
        }
    }

    fn spawnpoint_from_fields(fields: &Vec<FieldInstance>) -> Result<SpawnPoint, SPError> {
        let field = match fields.iter().find(|e| e.identifier == SPAWNPOINT_IDENT) {
            Some(v) => v,
            None => return Err(SPError::FieldNotFound),
        };

        if let FieldValue::String(v) = field.value.clone() {
            let inner_value = match v {
                Some(inner_v) => inner_v,
                None => return Err(SPError::EmptyInner),
            };

            return SpawnPoint::try_from(inner_value);
        }

        Err(SPError::InvalidFormat)
    }

    // END SYS

    // START STRUCT
    #[derive(Component, Default)]
    struct CollisionEntity;

    #[derive(Default, Bundle)]
    struct GrassTerrainBundle {
        marker: CollisionEntity,
        collider: Collider,
        rigid_body: RigidBody,
    }

    #[derive(Resource, Deref)]
    pub struct SpawnPoint(pub GridCoords);

    #[derive(EnumIter, Debug, Clone, Copy)]
    pub enum SPError {
        InvalidFormat,
        EmptyInner,
        FieldNotFound,
        NotEnoughFields,
    }

    impl Display for SPError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // TODO make this better wtf
            write!(f, "SPError::{:#?} [subcode={}]", self, *self as usize)
        }
    }

    impl TryFrom<String> for SpawnPoint {
        type Error = SPError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            let fields = value.split(';').collect::<Vec<&str>>();

            if fields.len() < 2 {
                return Err(SPError::NotEnoughFields);
            }

            bevy::log::debug!("{:?}", fields);

            let x = match str::parse::<i32>(fields[0]) {
                Ok(v) => v,
                Err(_) => return Err(SPError::InvalidFormat),
            };
            let y = match str::parse::<i32>(fields[1]) {
                Ok(v) => v,
                Err(_) => return Err(SPError::InvalidFormat),
            };

            Ok(SpawnPoint(GridCoords::new(x, y)))
        }
    }

    impl GrassTerrainBundle {
        fn new(collider: Collider, rigid_body: RigidBody) -> Self {
            Self {
                marker: CollisionEntity,
                collider,
                rigid_body,
            }
        }
    }

    impl LdtkIntCell for GrassTerrainBundle {
        fn bundle_int_cell(
            _: bevy_ecs_ldtk::IntGridCell,
            layer_instance: &bevy_ecs_ldtk::prelude::LayerInstance,
        ) -> Self {
            let collider = Collider::rectangle(
                layer_instance.grid_size as f32,
                layer_instance.grid_size as f32,
            );

            Self::new(collider, RigidBody::Static)
        }
    }

    // END STRUCT
}
