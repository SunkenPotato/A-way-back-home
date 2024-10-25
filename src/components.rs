pub mod component {

    use bevy::{math::{Vec2, Vec3}, prelude::{Component, ReflectComponent, With}, reflect::Reflect, time::Timer};

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct Velocity(pub Vec2);

    #[derive(Component, Reflect, Clone, Debug, Hash, PartialEq, Eq)]
    #[reflect(Component)]
    pub struct Identifier(pub String);

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct Tile;

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct SpriteMarker;

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct MovementMultiplier(Vec3);

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct Health {
        pub default: f32,
        pub current: f32
    }

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct Damage(pub f32); // Could be an event!

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct Mortal;

    #[derive(Component, Reflect)]
    #[reflect(Component)]
    pub struct AnimationConfig {
        pub first_sprite: usize,
        pub last_sprite: usize,
        pub fps: u8,
        pub frame_timer: Timer,
    }

    pub type WithSprite = With<SpriteMarker>;

    pub mod impls {
        use std::{fmt::Display, ops::{Add, Deref, Div, Mul, Sub}, time::Duration};

        use bevy::{math::Vec3, time::Timer};

        use super::{AnimationConfig, Damage, Health, MovementMultiplier};

        impl Default for MovementMultiplier {
            fn default() -> Self {
                Self(Vec3::from_slice(&[10., 1., 1.]))
            }
        }

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
                    current: self.current.mul(rhs)
                }
            }
        }

        impl Div<f32> for Health {
            type Output = Health;

            #[inline]
            fn div(self, rhs: f32) -> Self::Output {
                Self {
                    default: self.default,
                    current: self.current.div(rhs)
                }
            }
        }

        impl Add<f32> for Health {
            type Output = Health;
            
            #[inline]
            fn add(self, rhs: f32) -> Self::Output {
                Self {
                    default: self.default,
                    current: self.current.add(rhs)
                }
            }
        }

        impl Sub<f32> for Health {
            type Output = Health;
            
            #[inline]
            fn sub(self, rhs: f32) -> Self::Output {
                Self {
                    default: self.default,
                    current: self.current.sub(rhs)
                }
            }
        }

        impl From<(f32, f32)> for Health {
            fn from(value: (f32, f32)) -> Self {
                Self {
                    default: value.0,
                    current: value.1
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
                write!(f, "AnimationConfig {{\n\tfirst_sprite: {},\n\tlast_sprite: {},\n\tfps: {},\n\tframe_timer: {:#?}}}", self.first_sprite, self.last_sprite, self.fps, self.frame_timer)
            }
        }

        impl Health {
            pub fn is_dead(&self) -> bool {
                self.current < 0.
            }
        }

        impl AnimationConfig {
            pub fn new(first: usize, last: usize, fps: u8) -> Self {
                Self {
                    first_sprite: first,
                    last_sprite: last,
                    fps,
                    frame_timer: Self::timer_from_fps(fps)
                }
            }

            pub fn timer_from_fps(fps: u8) -> Timer {
                Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), bevy::time::TimerMode::Once)
            }
        }

    }
    
}

pub mod asset {

    use std::{io, ops::Deref};

    use bevy::{app::Plugin, asset::{Asset, AssetApp, AssetLoader, AsyncReadExt}, reflect::Reflect, utils::hashbrown::HashMap};

    #[derive(Reflect, Asset)]
    pub struct IndexAsset(pub HashMap<String, String>);

    impl Deref for IndexAsset {
        type Target = HashMap<String, String>;
        
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[derive(Default, Clone, Copy)]
    pub struct IndexAssetLoader;

    pub struct AssetPlugin;

    impl Plugin for AssetPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.init_asset::<IndexAsset>();
            app.init_asset_loader::<IndexAssetLoader>();
        }
    }

    impl AssetLoader for IndexAssetLoader {
        type Asset = IndexAsset;
        type Error = io::Error;
        type Settings = ();

        fn load<'a>(
                &'a self,
                reader: &'a mut bevy::asset::io::Reader,
                _settings: &'a Self::Settings,
                load_context: &'a mut bevy::asset::LoadContext,
            ) -> impl bevy::utils::ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
            
                Box::pin(async move {
                    let mut buf = String::new();
                    reader.read_to_string(&mut buf).await.expect("Could not read TextAsset");
                    
                    Ok(IndexAsset(serde_json::from_str(&buf).unwrap_or_else(|_| panic!("Invalid JSON in asset: {}", load_context.asset_path()))))
                })

        }

        fn extensions(&self) -> &[&str] {
            &["txt", "json", ""]
        }
    }

}