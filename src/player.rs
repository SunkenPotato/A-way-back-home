
use bevy::{asset::AssetServer, math::Vec3, prelude::{default, Commands, Component, Query, Res, Transform}, sprite::SpriteBundle, time::{Time, Timer}};


const SCALE_AMOUNT: f32 = 2.5;
const SCALE: Vec3 = Vec3 { x: SCALE_AMOUNT, y: SCALE_AMOUNT, z: SCALE_AMOUNT };


#[derive(Component)]
pub struct Player {
    pub facing: Direction,
    pub v_elocity: f32,
    pub jump_start_time: f32,
    pub is_grounded: bool
}

pub enum Direction {
    L,
    R
}

/*
fn height(initial_velocity: f32, time: f32) -> f32 {
    let r = initial_velocity * time - (GRAVITATIONAL_ACCELERATION * f32::powi(time, 2)) / 2.0;
    dbg!(r);
    r
}
*/

pub fn move_right(mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in &mut query {
        player.facing = Direction::R;
        transform.translation.x += 1.;
        transform.scale = transform.scale.with_x(SCALE_AMOUNT);
    }
}

pub fn move_left(mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in &mut query {
        player.facing = Direction::L;
        transform.translation.x -= 1.;
        transform.scale = transform.scale.with_x(-SCALE_AMOUNT);
    }
}

pub fn jump(time: Res<Time>, mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in &mut query {
        if player.is_grounded {
            player.v_elocity = 100.0;
            player.is_grounded = false;
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player {
            facing: Direction::L,
            v_elocity: 0.,
            jump_start_time: 0.,
            is_grounded: true
        },
        SpriteBundle {
            texture: asset_server.load("sprites/claire-left.png"),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(SCALE),
            ..default()
        }
    ));
}