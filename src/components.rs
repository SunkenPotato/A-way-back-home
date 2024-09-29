use avian2d::prelude::Collider;
use bevy::{
    math::Vec2,
    prelude::{Component, FromWorld, ReflectComponent, Resource},
    reflect::Reflect,
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Moveable;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Grounded(pub bool);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Killable;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub direction: Direction,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Tile {
    pub(crate) pos: Vec2,
    pub(crate) size: Vec2,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpriteMarker(pub String);

impl Health {
    pub fn is_dead(&self) -> bool {
        self.0 <= 0.
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Speed {
    speed: f32,
    multiplier: f32,
}

impl Speed {
    pub fn default(s: f32) -> Self {
        Self {
            speed: s,
            multiplier: 1.,
        }
    }

    pub fn with_multiplier(mut self, m: f32) -> Self {
        self.multiplier = m;
        self
    }

    pub fn speed(&self) -> f32 {
        self.speed * self.multiplier * 200.
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub enum Direction {
    L,
    #[default]
    R,
}

#[derive(Resource)]
pub struct GameOver(pub bool);

impl FromWorld for Health {
    fn from_world(_world: &mut bevy::prelude::World) -> Self {
        Self(20.)
    }
}

impl FromWorld for Tile {
    fn from_world(_world: &mut bevy::prelude::World) -> Self {
        Self {
            pos: Vec2::splat(0.),
            size: Vec2 {
                x: crate::world::terrain::TILE_SIZE.0,
                y: crate::world::terrain::TILE_SIZE.0,
            },
        }
    }
}

impl FromWorld for Velocity {
    fn from_world(_world: &mut bevy::prelude::World) -> Self {
        Self(Vec2::ZERO)
    }
}

impl FromWorld for Killable {
    fn from_world(_world: &mut bevy::prelude::World) -> Self {
        Self
    }
}

impl FromWorld for Grounded {
    fn from_world(_world: &mut bevy::prelude::World) -> Self {
        Self(true)
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            direction: Direction::default(),
        }
    }
}
