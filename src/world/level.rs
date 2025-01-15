use std::sync::LazyLock;

use bevy::prelude::{Commands, Event, EventReader, EventWriter, ResMut};
use bevy_ecs_ldtk::LevelSelection;

use crate::player::SyncCameraWithPlayer;

// TODO: remove
static FIRST_LEVEL: LazyLock<LevelSelection> =
    LazyLock::new(|| LevelSelection::iid("977043b0-c210-11ef-82b1-d58aa0d63a63"));

#[derive(Event)]
pub struct ChangeLevel(pub LevelSelection);

pub(super) fn initial_level_change(mut commands: Commands) {
    commands.insert_resource(FIRST_LEVEL.clone());
    commands.send_event(ChangeLevel(FIRST_LEVEL.clone()));
}

pub(super) fn change_level(
    mut event_reader: EventReader<ChangeLevel>,
    mut level_selection: ResMut<LevelSelection>,
    mut event_writer: EventWriter<SyncCameraWithPlayer>,
) {
    let Some(last) = event_reader.read().last() else {
        return;
    };

    *level_selection = last.0.clone();
    event_writer.send_default();
}
