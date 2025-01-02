use bevy::app::plugin_group;

mod camera;

plugin_group! {
    pub struct RenderPlugins {
        camera:::CameraPlugin
    }
}
