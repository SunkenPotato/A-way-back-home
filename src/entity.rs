use bevy::{prelude::{Commands, Entity, Query, Res, Transform, With}, time::Time};

use crate::components::{Grounded, Health, Killable, Moveable, Speed, Velocity};

pub fn moveable_system(mut query: Query<(&Velocity, &mut Transform, &Speed), With<Moveable>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (velocity, mut transform, speed) in &mut query {
        let translation = &mut transform.translation;

        let added_v_x = velocity.0.x * delta * speed.speed();
        let added_v_y = velocity.0.y * delta * speed.speed();

        translation.x += added_v_x;
        translation.y += added_v_y;
    }
}

pub fn grounded_system(mut query: Query<(&mut Grounded, &Velocity), With<Moveable>>) {
    for (mut grounded, velocity) in &mut query {
        if velocity.0.y == 0. {
            grounded.0 = true;
        } else {
            grounded.0 = false;
        }
    }
}

pub fn death_system(mut commands: Commands, query: Query<(&Health, Entity), With<Killable>>) {
    for (health, entity) in &query {
        if health.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}