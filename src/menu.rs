pub mod bindings {
    use bevy::{
        app::{AppExit, PreUpdate},
        input::ButtonInput,
        prelude::{EventWriter, KeyCode, Res},
    };

    pub struct KeybindPlugin;

    impl bevy::app::Plugin for KeybindPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.add_systems(PreUpdate, quit_game);
        }
    }

    type KeyboardInput<'w> = Res<'w, ButtonInput<KeyCode>>;

    fn quit_game(keyboard_input: KeyboardInput, mut quit_game: EventWriter<AppExit>) {
        if keyboard_input.pressed(KeyCode::ControlLeft) && keyboard_input.pressed(KeyCode::KeyQ) {
            quit_game.send(AppExit::Success);
            bevy::log::info!("Ctrl+Q received, exiting.")
        }
    }
}
