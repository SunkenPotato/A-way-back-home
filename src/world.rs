pub mod loader {

    use std::{num::NonZero, sync::LazyLock};

    use avian2d::prelude::{Collider, RigidBody};
    use bevy::{
        app::{Plugin, Startup, Update},
        asset::{AssetServer, Assets, Handle},
        math::Vec3,
        prelude::{
            Bundle, Commands, Component, Event, EventReader, EventWriter, Query, Res, Resource,
        },
        utils::default,
    };
    use bevy_ecs_ldtk::{
        app::{LdtkIntCell, LdtkIntCellAppExt},
        assets::{LdtkProject, LevelMetadataAccessor},
        ldtk::{FieldInstance, FieldValue},
        GridCoords, LdtkWorldBundle, LevelIid, LevelSelection,
    };

    pub struct WorldPlugin;

    pub static EXIT_ERROR_CODE: LazyLock<NonZero<u8>> = LazyLock::new(|| NonZero::new(1).unwrap());

    pub const GLOBAL_SCALE: Vec3 = Vec3::from_slice(&[3.5, 3.5, 0.]);

    const GRASS_INT_CELL: i32 = 1;

    const SPAWNPOINT_IDENT: &'static str = "Spawnpoint";

    #[derive(Event)]
    pub struct ChangeLevel(usize);

    impl Plugin for WorldPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_systems(Startup, load_world);
            app.add_systems(Update, set_level);
            app.add_event::<ChangeLevel>();
            app.register_ldtk_int_cell::<GrassTerrainBundle>(GRASS_INT_CELL);

            // remove!
            app.add_systems(Update, get_level_spawnpoint);
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
    fn set_level(
        mut commands: Commands,
        mut change_level_e: EventReader<ChangeLevel>,
        iid_query: Query<&LevelIid>,
        project_query: Query<&Handle<LdtkProject>>,
        ldtkp_assets: Res<Assets<LdtkProject>>,
    ) {
        for event in change_level_e.read() {
            commands.insert_resource(LevelSelection::index(event.0));

            for level_iid in &iid_query {
                let single_project = ldtkp_assets.get(project_query.single()).unwrap();

                let level = match single_project.get_raw_level_by_iid(level_iid.get()) {
                    Some(v) => v,
                    None => {
                        crate::error::errors::LEVEL_NOT_FOUND.trigger();
                        return;
                    }
                };

                let fields = level.field_instances.clone();

                let spawn_point = todo!();
            }
        }
    }

    // REFERNCE for future
    /**
    fn get_spawnpoint_from_fields(
        fields: &Vec<FieldInstance>,
    ) -> Option<Result<SpawnPoint, SPError>> {
        for field in fields.iter() {
            if field.identifier == SPAWNPOINT_IDENT {
                match field.value {
                    FieldValue::String(v) => match v {
                        Some(v) => v,
                        Err(_) => return Some(Err("Expected a non-empty field!")),
                    },
                    _ => return Ok(None),
                }
            }
        }

        None
    }**/

    fn get_level_spawnpoint(
        query: Query<&LevelIid>,
        projects: Query<&Handle<LdtkProject>>,
        assets: Res<Assets<LdtkProject>>,
    ) {
        for level_iid in &query {
            let only_project = assets.get(projects.single()).unwrap();

            let level = only_project.get_raw_level_by_iid(level_iid.get()).unwrap();
            let fields = level.field_instances.clone();
        }
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

    #[derive(Resource)]
    pub struct SpawnPoint(GridCoords);

    enum SPError {
        InvalidFormat,
        NotEnoughFields,
    }

    impl TryFrom<String> for SpawnPoint {
        type Error = SPError;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            let fields = value.split(';').collect::<Vec<&str>>();

            if fields.len() < 2 {
                return Err(SPError::NotEnoughFields);
            }

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
