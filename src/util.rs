use avian2d::prelude::PhysicsDebugPlugin;
use bevy::{
    app::App,
    log::{Level, LogPlugin},
};
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

const PHYSICS_DEBUG: u32 = 0b001;
const LOG_DEBUG: u32 = 0b010;

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

pub fn debug_mode(app: &mut App, log_plugin: &mut LogPlugin) {
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

    if (bitflags | PHYSICS_DEBUG) > 0 {
        app.add_plugins(PhysicsDebugPlugin::default());
    }

    if (bitflags | LOG_DEBUG) > 0 {
        log_plugin.level = Level::DEBUG;
    }
}

pub mod convert {
    use std::convert::Infallible;

    use bevy::math::{IVec2, Vec2, Vec3};
    use bevy_ecs_ldtk::GridCoords;

    // BEGIN - trait definitions
    pub trait LocalFrom<T>: Sized {
        #[must_use]
        fn local_from(value: T) -> Self;
    }

    pub trait LocalInto<T>: Sized {
        #[must_use]
        fn local_into(self) -> T;
    }

    pub trait LTryFrom<T>: Sized {
        type Error;

        #[must_use]
        fn l_try_from(value: T) -> Result<Self, Self::Error>;
    }

    pub trait LTryInto<T>: Sized {
        type Error;

        #[must_use]
        fn l_try_into(self) -> Result<T, Self::Error>;
    }

    // END - trait definitions

    // BEGIN - generic impls
    impl<T, U> LocalInto<U> for T
    where
        U: LocalFrom<T>,
    {
        #[inline]
        fn local_into(self) -> U {
            U::local_from(self)
        }
    }

    impl<T> LocalFrom<T> for T {
        #[inline]
        fn local_from(value: T) -> Self {
            value
        }
    }

    impl<T, U> LTryInto<U> for T
    where
        U: LTryFrom<T>,
    {
        type Error = U::Error;

        #[inline]
        fn l_try_into(self) -> Result<U, Self::Error> {
            U::l_try_from(self)
        }
    }

    impl<T, U> LTryFrom<U> for T
    where
        U: Into<T>,
    {
        type Error = Infallible;

        fn l_try_from(value: U) -> Result<Self, Self::Error> {
            Ok(U::into(value))
        }
    }

    // END - generic impls

    impl LocalFrom<Vec3> for Vec2 {
        #[inline]
        fn local_from(value: Vec3) -> Self {
            Self {
                x: value.x,
                y: value.y,
            }
        }
    }

    impl LocalFrom<Vec3> for bevy::a11y::accesskit::Vec2 {
        #[inline]
        fn local_from(value: Vec3) -> Self {
            Self {
                x: value.x as f64,
                y: value.y as f64,
            }
        }
    }

    impl LocalFrom<(f32, f32)> for (i32, i32) {
        /// Performs a lossy conversion
        fn local_from(value: (f32, f32)) -> Self {
            (value.0 as i32, value.1 as i32)
        }
    }

    pub fn grid_coords_from_vec3(translation: Vec3, tile_size: IVec2) -> GridCoords {
        let vec3_tile_size = tile_size.as_vec2().extend(1.);

        let relative_translation = translation - (vec3_tile_size / 2.);

        let tile_coords = (relative_translation / vec3_tile_size).round();

        GridCoords {
            x: tile_coords.x as i32,
            y: tile_coords.y as i32,
        }
    }
}
