use bevy::prelude::{Commands, Component};

pub trait SystemDefault {
    fn sdefault() -> Self;
}

pub fn spawn_default<T>(mut commands: Commands)
where
    T: Default + Component,
{
    commands.spawn(T::default());
}

pub fn spawn_sdefault<T>(mut commands: Commands)
where
    T: SystemDefault + Component,
{
    commands.spawn(T::sdefault());
}
