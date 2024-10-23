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
