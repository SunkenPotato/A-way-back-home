use bevy::app::plugin_group;

pub mod animation;
mod camera;

plugin_group! {
    pub struct RenderPlugins {
        camera:::CameraPlugin,
        animation:::AnimationPlugin
    }
}
