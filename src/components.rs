pub mod component {
    use bevy::{
        math::Vec3,
        prelude::{Component, ReflectComponent},
        reflect::Reflect,
        time::Timer,
    };

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct MovementMultiplier(Vec3);

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct Health {
        pub default: f32,
        pub current: f32,
    }

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct Damage(pub f32); // Could be an event!

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct Mortal;

    #[derive(Component, Reflect, Debug)]
    #[reflect(Component)]
    pub struct AnimationConfig {
        pub sprite_indices: SpriteIndices,
        pub fps: u8,
        pub frame_timer: Timer,
    }

    #[derive(Component, Reflect, Debug)]
    #[reflect(Component)]
    pub struct SpriteIndices {
        pub first_sprite: usize,
        pub last_sprite: usize,
    }

    #[derive(Component)]
    pub struct Animatable;

    pub mod impls {
        use std::{
            fmt::Display,
            ops::{Add, Deref, Div, Mul, Sub},
            time::Duration,
        };

        use bevy::{math::Vec3, time::Timer};

        use super::{AnimationConfig, Damage, Health, MovementMultiplier, SpriteIndices};

        crate::default_impl!(MovementMultiplier, Self(Vec3::from_slice(&[10., 1., 1.])));

        impl Deref for MovementMultiplier {
            type Target = Vec3;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Deref for Damage {
            type Target = f32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Mul<f32> for Health {
            type Output = Health;

            #[inline]
            fn mul(self, rhs: f32) -> Self::Output {
                Self {
                    default: self.default,
                    current: self.current.mul(rhs),
                }
            }
        }

        impl Div<f32> for Health {
            type Output = Health;

            #[inline]
            fn div(self, rhs: f32) -> Self::Output {
                Self {
                    default: self.default,
                    current: self.current.div(rhs),
                }
            }
        }

        impl Add<f32> for Health {
            type Output = Health;

            #[inline]
            fn add(self, rhs: f32) -> Self::Output {
                Self {
                    default: self.default,
                    current: self.current.add(rhs),
                }
            }
        }

        impl Sub<f32> for Health {
            type Output = Health;

            #[inline]
            fn sub(self, rhs: f32) -> Self::Output {
                Self {
                    default: self.default,
                    current: self.current.sub(rhs),
                }
            }
        }

        impl From<(f32, f32)> for Health {
            fn from(value: (f32, f32)) -> Self {
                Self {
                    default: value.0,
                    current: value.1,
                }
            }
        }

        impl Display for MovementMultiplier {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
            }
        }

        impl Display for AnimationConfig {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let sprite_indices = &self.sprite_indices;
                write!(f, "AnimationConfig {{\n\tfirst_sprite: {},\n\tlast_sprite: {},\n\tfps: {},\n\tframe_timer: {:#?}}}", sprite_indices.first_sprite, sprite_indices.last_sprite, self.fps, self.frame_timer)
            }
        }

        impl Health {
            pub const fn new(amount: f32) -> Self {
                Self {
                    default: amount,
                    current: amount,
                }
            }
            pub fn is_dead(&self) -> bool {
                self.current < 0.
            }
        }

        impl AnimationConfig {
            pub fn new(sprite_indices: SpriteIndices, fps: u8) -> Self {
                Self {
                    sprite_indices,
                    fps,
                    frame_timer: Self::timer_from_fps(fps),
                }
            }

            pub fn timer_from_fps(fps: u8) -> Timer {
                Timer::new(
                    Duration::from_secs_f32(1.0 / (fps as f32)),
                    bevy::time::TimerMode::Once,
                )
            }
        }

        impl SpriteIndices {
            pub const fn new(first_sprite: usize, last_sprite: usize) -> Self {
                Self {
                    first_sprite,
                    last_sprite,
                }
            }
        }
    }
}
