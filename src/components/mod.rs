use bevy::{
    app::{Plugin, Update},
    prelude::{Changed, Component, Query},
    sprite::Sprite,
};

pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, flip_sprite);
    }
}

#[derive(Component, Default)]
pub enum EntityDirection {
    L,
    #[default]
    R,
}

fn flip_sprite(mut query: Query<(&mut Sprite, &EntityDirection), Changed<EntityDirection>>) {
    for (mut sprite, direction) in &mut query {
        match direction {
            EntityDirection::L => sprite.flip_x = true,
            EntityDirection::R => sprite.flip_x = false,
        }
    }
}
