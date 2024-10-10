pub mod component {

    use bevy::{math::{Vec2, Vec3}, prelude::{Component, ReflectComponent, With}, reflect::Reflect};

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

    pub type WithSprite = With<SpriteMarker>;

    pub mod impls {
        use std::ops::Deref;

        use bevy::math::Vec3;

        use super::MovementMultiplier;

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
                    
                    Ok(IndexAsset(serde_json::from_str(&buf).expect(&format!("Invalid JSON in asset: {}", load_context.asset_path()))))
                })

        }

        fn extensions(&self) -> &[&str] {
            &["txt", "json", ""]
        }
    }

}