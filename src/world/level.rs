use bevy::{
    log::warn,
    prelude::{Bundle, Component, Event, EventReader, EventWriter, Query, ResMut, With},
    sprite::Sprite,
};
use bevy_ecs_ldtk::{GridCoords, LdtkEntity, LevelSelection};

use crate::{
    impl_entity,
    player::{Player, SyncCameraWithPlayer},
    query_as_single,
};

#[derive(Event, Debug, Default)]
pub enum ChangeLevel {
    Level(LevelSelection),
    #[default]
    Next,
}

impl ChangeLevel {
    pub fn level(level: LevelSelection) -> Self {
        Self::Level(level)
    }
}

#[derive(Component, Default)]
pub struct LevelGoal;

#[derive(Default, Bundle, LdtkEntity)]
pub(super) struct LevelGoalBundle {
    _marker: LevelGoal,
    #[cfg(debug_assertions)]
    #[sprite_sheet]
    sprite: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

impl_entity!(LevelGoalBundle | "LevelGoal"; 16.);

pub(super) fn change_level(
    mut event_reader: EventReader<ChangeLevel>,
    mut level_selection: ResMut<LevelSelection>,
    mut event_writer: EventWriter<SyncCameraWithPlayer>,
) {
    for level in event_reader.read() {
        let LevelSelection::Indices(indices) = level_selection.as_ref() else {
            warn!("LevelSelection should be of `Indices` form");
            return;
        };

        let new_level = match level {
            ChangeLevel::Next => LevelSelection::index(indices.level + 1),
            ChangeLevel::Level(inner) => inner.clone(),
        };
        *level_selection = new_level;
        event_writer.send_default();
    }
}

// Reminder: change level_goal to Option<T> possibly.
pub(super) fn transition_level(
    player: Query<&GridCoords, With<Player>>,
    level_goal: Query<&GridCoords, With<LevelGoal>>, // could be cached
    mut change_level: EventWriter<ChangeLevel>,
) {
    query_as_single!(player; player);
    query_as_single!(level_goal; level_goal);

    let player = GridCoords::new(player.x, player.y - 1); // because the player doesn't fit on the normal grid

    if &player != level_goal {
        return;
    }
    println!("changing");
    change_level.send_default();
}
