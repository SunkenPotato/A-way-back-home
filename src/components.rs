use bevy::{math::Vec2, prelude::{Component, Resource}};

#[derive(Component)]
pub struct Moveable;

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct Killable;

#[derive(Component)]
pub struct Velocity(pub Vec2);  

#[derive(Component, Default)]
pub struct Player {
    pub direction: Direction,
}

#[derive(Component)]
pub struct Health(pub f32);

impl Health {
    pub fn is_dead(&self) -> bool {
        self.0 <= 0.
    }
}

#[derive(Component)]
pub struct Speed {
    speed: f32,
    multiplier: f32
}

#[derive(Component, Default)]
pub enum Direction {
    L,
    #[default]
    R
}

#[derive(Resource)]
pub struct GameOver(pub bool);

impl Speed {
    pub fn default(s: f32) -> Self {
        Self {
            speed: s,
            multiplier: 1.
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