use bevy::{math::Vec2, prelude::{Query, Res, Transform, With, Without}, sprite::Sprite, time::Time};

use crate::player::Player;

const GRAVITATIONAL_ACCELERATION: f32 = 100.0;

pub fn gravity_s(time: Res<Time>, mut query: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in &mut query {
        player.v_elocity -= GRAVITATIONAL_ACCELERATION * time.delta_seconds();

        player.v_elocity = player.v_elocity.max(-50.0);

        transform.translation.y += player.v_elocity * time.delta_seconds();

        if transform.translation.y <= 0.0 {
            transform.translation.y = 0.0;
            player.is_grounded = true;
            player.v_elocity = 0.0;
        }
    }
}
pub fn collision(
    mut player_query: Query<(&Sprite, &mut Transform), With<Player>>,
    collider_query: Query<(&Transform, &Sprite), Without<Player>>,
) {
    for (_, mut player_transform) in &mut player_query {
        let player_size = Vec2 { x: 16., y: 16. };

        let player_min = player_transform.translation.truncate() - player_size / 2.0;
        let player_max = player_transform.translation.truncate() + player_size / 2.0;

        for (collider_transform, collider_sprite) in &collider_query {
            let collider_size = match collider_sprite.custom_size {
                Some(size) => size,
                None => continue,
            };

            let collider_min = collider_transform.translation.truncate() - collider_size / 2.0;
            let collider_max = collider_transform.translation.truncate() + collider_size / 2.0;

            if player_min.x < collider_max.x
                && player_max.x > collider_min.x
                && player_min.y < collider_max.y
                && player_max.y > collider_min.y
            {
                bevy::log::info!("Not something...?");
                player_transform.translation.y = collider_max.y + player_size.y / 2.0;
            } else {
                bevy::log::info!("Something...?")
            }
        }
    }
    
}