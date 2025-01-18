use bevy::{
    app::{Plugin, Update},
    math::IVec2,
    prelude::{Changed, Component, Query, Transform},
    sprite::Sprite,
};
use bevy_ecs_ldtk::GridCoords;

pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (flip_sprite, update_grid_coords_from_transform));
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

fn update_grid_coords_from_transform(
    mut query: Query<(&Transform, &mut GridCoords), Changed<Transform>>,
) {
    for (transform, mut grid_coords) in &mut query {
        *grid_coords = bevy_ecs_ldtk::utils::translation_to_grid_coords(
            transform.translation.truncate(),
            IVec2::new(16, 16),
        );
    }
}
