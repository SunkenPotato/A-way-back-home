// TODO: Implement death system
pub mod health {
    use std::ops::Deref;

    use bevy::{
        app::{Plugin, Update},
        prelude::{Commands, Entity, Query, Res, Resource, Transform, With, Without},
    };

    use crate::{
        components::component::{Health, Mortal},
        player::Player,
    };

    pub struct HealthPlugin;

    #[derive(Resource)]
    pub struct VoidHeight(f32);

    impl Default for VoidHeight {
        fn default() -> Self {
            Self(-300.)
        }
    }

    impl Deref for VoidHeight {
        type Target = f32;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Plugin for HealthPlugin {
        fn build(&self, app: &mut bevy::prelude::App) {
            app.init_resource::<VoidHeight>();
            app.add_systems(Update, (void_death_system, kill_system));
        }
    }

    #[allow(clippy::type_complexity)]
    // Player gets its own special kill system
    fn void_death_system(
        mut entities: Query<(&Transform, &mut Health), (Without<Player>, With<Mortal>)>,
        void_height: Res<VoidHeight>,
    ) {
        for (transform, mut health) in &mut entities {
            if transform.translation.y <= **void_height {
                health.current = 0.;
            }
        }
    }

    // Loot tables maybe?
    fn kill_system(
        mut commands: Commands,
        e: Query<(&Health, Entity), (Without<Player>, With<Mortal>)>,
    ) {
        e.iter().for_each(move |(h, e)| {
            if h.is_dead() {
                commands.entity(e).despawn();
            }
        });
    }
}
