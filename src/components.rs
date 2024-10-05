pub mod component {
    use bevy::{math::Vec2, prelude::{Component, ReflectComponent, With}, reflect::Reflect};

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct Velocity(pub Vec2);

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct Identifier<'a>(pub &'a str);

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct Tile;

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct SpriteMarker;

    pub type WithSprite = With<SpriteMarker>;
    
}