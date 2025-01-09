use bevy::{
    asset::Assets,
    log::error,
    prelude::{DetectChanges, Res, ResMut, Resource, Single},
};
use bevy_ecs_ldtk::{
    assets::{LdtkProject, LevelMetadataAccessor},
    ldtk::{FieldInstance, FieldValue},
    LdtkProjectHandle, LevelSelection,
};

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

#[derive(Default, Debug)]
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

    let LevelSelection::Iid(ref iid) = *level_selection else {
        error!("Level identifier should be of IID form");
        return;
    };

    let level = project
        .get_raw_level_by_iid(iid.get())
        .expect("level should exist as per inserted iid");

    *level_settings = LevelSettings::from_field_instances(&level.field_instances);
}
