use bevy::{
    asset::Assets,
    prelude::{DetectChanges, Res, ResMut, Resource, Single},
};
use bevy_ecs_ldtk::{
    assets::{LdtkProject, LevelMetadataAccessor},
    ldtk::{FieldInstance, FieldValue},
    prelude::RawLevelAccessor,
    LdtkProjectHandle, LevelSelection,
};
use derive_more::derive::Deref;

trait FromFieldInstances: FromFieldValue {
    const IDENTIFIER: &'static str;

    fn from_field_instances(f: &Vec<FieldInstance>) -> Option<Self> {
        Self::from_field_value(
            f.iter()
                .find(|e| e.identifier == Self::IDENTIFIER)?
                .value
                .clone(),
        )
    }
}

trait FromFieldValue: Sized {
    fn from_field_value(val: FieldValue) -> Option<Self>;
}

#[derive(Resource, Default, Debug)]
pub struct LevelSettings {
    pub camera_follow: CameraFollow,
}

impl LevelSettings {
    pub fn from_field_instances(fi: &Vec<FieldInstance>) -> Self {
        Self {
            camera_follow: CameraFollow::from_field_instances(&fi).unwrap(),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Deref)]
pub struct CameraFollow(pub bool);
impl FromFieldInstances for CameraFollow {
    const IDENTIFIER: &'static str = "CameraFollow";
}

impl FromFieldValue for CameraFollow {
    fn from_field_value(val: FieldValue) -> Option<Self> {
        let FieldValue::Bool(v) = val else {
            return None;
        };
        Some(Self(v))
    }
}

pub(super) fn update_level_settings(
    mut level_settings: ResMut<LevelSettings>,
    level_selection: Res<LevelSelection>,
    handle: Single<&LdtkProjectHandle>,
    projects: Res<Assets<LdtkProject>>,
) {
    if !level_selection.is_changed() {
        return;
    }

    let project: &LdtkProject = projects.get(&handle.handle).expect("project should exist");

    let level = match *level_selection {
        LevelSelection::Indices(ref idx) => project.get_raw_level_at_indices(&idx),
        LevelSelection::Iid(ref iid) => project.get_raw_level_by_iid(iid.get()),
        _ => todo!(),
    }
    .expect("level should exist");

    *level_settings = LevelSettings::from_field_instances(&level.field_instances);
}
