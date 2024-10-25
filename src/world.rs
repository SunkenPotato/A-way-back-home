pub mod loader {
    
    use std::{num::NonZero, sync::LazyLock};

    use bevy::{app::{Plugin, Startup, Update}, asset::AssetServer, math::Vec3, prelude::{Commands, Event, EventReader, EventWriter, Res}, utils::default};
    use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

    pub struct WorldPlugin;

    pub static EXIT_ERROR_CODE: LazyLock<NonZero<u8>> = LazyLock::new(|| NonZero::new(1).unwrap());

    pub const GLOBAL_SCALE: Vec3 = Vec3::from_slice(&[3.5, 3.5, 0.]);

    #[derive(Event)]
    pub struct ChangeLevel(usize);

    impl Plugin for WorldPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_systems(Startup, load_world);
            app.add_systems(Update, set_level);
            app.add_event::<ChangeLevel>();
        }
    }

    fn load_world(mut commands: Commands, asset_server: Res<AssetServer>, mut event_writer: EventWriter<ChangeLevel>) {
        let ldtk_handle = asset_server.load("scenes/test-world.ldtk");

        let ldtk_bundle = LdtkWorldBundle {
            ldtk_handle,
            ..default()
        };

        commands.spawn(ldtk_bundle);
        event_writer.send(ChangeLevel(0));
    }

    fn set_level(mut commands: Commands, mut change_level_e: EventReader<ChangeLevel>) {
        for event in change_level_e.read() {
            commands.insert_resource(LevelSelection::index(event.0));
        }
    }

}