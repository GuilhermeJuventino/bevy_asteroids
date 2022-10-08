use bevy::prelude::*;

use crate::components::LaserDespawnTimer;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(projectile_despawn_system);
    }
}

fn projectile_despawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut LaserDespawnTimer)>,
) {
    for (entity, mut despawn_timer) in query.iter_mut() {
        despawn_timer.0.tick(time.delta());

        if despawn_timer.0.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
