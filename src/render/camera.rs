use bevy::{
    app::{Plugin, Startup},
    math::Vec3,
    prelude::{Camera2d, Commands, Transform},
};

const CAMERA_SCALE: f32 = 1. / 5.;

#[derive(Default)]
pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_scale(Vec3::splat(CAMERA_SCALE))));
}
