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
    pub const VEC_G: Vec2 = Vec2 { x: 0., y: -ADJUSTED_G };
}   

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