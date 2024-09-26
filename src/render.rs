use bevy::{asset::{AssetServer, Handle}, math::{Vec2, Vec3}, prelude::{Camera2dBundle, Commands, Component, Image, Query, Res, ResMut, Transform}, render::render_resource::Texture, sprite::SpriteBundle, utils::default, window::Window};

#[derive(Component)]
struct CameraMarker;

pub fn draw_scene(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&mut Window>) {
    let window = windows.single();
    let sprite: Handle<Image> = asset_server.load("sprites/terrain/dirt-1.png");

    let sprite_size = Vec2::new(32., 32.);
    let horizontal_count = (window.width() / sprite_size.x).ceil() as i32;
    let vertical_count = (window.height() / sprite_size.y).ceil() as i32;

    for x in -horizontal_count..horizontal_count {
        commands.spawn(SpriteBundle {
            texture: sprite.clone(),
            transform: Transform::from_xyz(x as f32*sprite_size.x, -20., 0.).with_scale(Vec3 { x: 2., y: 2., z: 2. }),
            ..default()
        });
    }

}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        CameraMarker
    ));
}