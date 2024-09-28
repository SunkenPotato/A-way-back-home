// TODO:
// Create some test terrain, with moving platforms, possibly
// and some air-blocks :)\
// TODO, n.2.
// Fix rectangle to extend to edge of screen, i.e., compute window size properly


pub mod terrain {
    use avian2d::prelude::{Collider, RigidBody};
    use bevy::{asset::Assets, color::Color, prelude::{Commands, Mesh, Query, Rectangle, ResMut, Transform}, sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle}, utils::default, window::Window};
    
    #[allow(unused)]
    const TILE_SCALE: f32 = 4.;
    #[allow(unused)]
    const TILE_SIZE: (f32, f32) = (16. * TILE_SCALE, 16. * TILE_SCALE);

    pub fn setup_terrain(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, windows: Query<&Window>) {
        let window = windows.single();

        let width = window.width();
        
        let rect = Mesh2dHandle(meshes.add(Rectangle::new(width, 32.)));
        let color = Color::linear_rgb(1., 1., 0.);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: rect,
                material: materials.add(color),
                transform: Transform::from_xyz(0., -32., 0.),
                ..default()
            },
            Collider::rectangle(width, 32.),
            RigidBody::Static
        ));
    }
}