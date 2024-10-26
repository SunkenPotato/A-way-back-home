use avian2d::prelude::PhysicsDebugPlugin;
use bevy::app::App;
use bevy_ecs_ldtk::GridCoords;

pub mod consts {
    use bevy::math::Vec2;

    /// Earth's Gravitational constant (`g`)
    pub const G: f32 = 9.7803267715;
    /// A multiplier for `G` to make it significant in-game
    pub const G_MULTIPLIER: f32 = 100.;
    /// Fully calculated `G` adjusted for the game
    pub const ADJUSTED_G: f32 = G * G_MULTIPLIER;
    /// Downwards Vec2 representing the adjusted gravity (see `ADJUSTED_G`)
    pub const VEC_G: Vec2 = Vec2 {
        x: 0.,
        y: -ADJUSTED_G,
    };
}

const PHYSICS_DEBUG: u32 = 2_u32.pow(0);

pub trait GridCoordConst {
    const NEG_X: GridCoords;
    const X: GridCoords;
    const Y: GridCoords;
    const NEG_Y: GridCoords;
    const ZERO: GridCoords = GridCoords { x: 0, y: 0 };
}

impl GridCoordConst for GridCoords {
    const NEG_Y: GridCoords = GridCoords { x: 0, y: -1 };
    const Y: GridCoords = GridCoords { x: 0, y: 1 };
    const NEG_X: GridCoords = GridCoords { x: -1, y: 0 };
    const X: GridCoords = GridCoords { x: 1, y: 0 };
}

pub fn debug_mode(app: &mut App) {
    let Some(flags) = option_env!("DEBUG") else {
        return;
    };
    let bitflags = match flags.parse::<u32>() {
        Ok(v) => v,
        Err(e) => {
            bevy::log::error!(
                "Could not parse debug environment value from compilation time to a u32: {e}"
            );
            return;
        }
    };

    if (bitflags & PHYSICS_DEBUG) > 0 {
        app.add_plugins(PhysicsDebugPlugin::default());
    }
}

pub mod convert {
    use bevy::math::{Vec2, Vec3};

    // BEGIN - trait definitions
    pub trait LocalFrom<T>: Sized {
        #[must_use]
        fn from(value: T) -> Self;
    }

    pub trait LocalInto<T>: Sized {
        #[must_use]
        fn into(self) -> T;
    }
    // END - trait definitions

    // BEGIN - generic impls
    impl<T, U> LocalInto<U> for T
    where
        U: LocalFrom<T>,
    {
        #[inline]
        fn into(self) -> U {
            U::from(self)
        }
    }

    impl<T> LocalFrom<T> for T {
        #[inline]
        fn from(value: T) -> Self {
            value
        }
    }

    // END - generic impls

    impl LocalFrom<Vec3> for Vec2 {
        #[inline]
        fn from(value: Vec3) -> Self {
            Self {
                x: value.x,
                y: value.y,
            }
        }
    }

    impl LocalFrom<Vec3> for bevy::a11y::accesskit::Vec2 {
        #[inline]
        fn from(value: Vec3) -> Self {
            Self {
                x: value.x as f64,
                y: value.y as f64,
            }
        }
    }
}
