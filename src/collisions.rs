use std::collections::HashSet;

use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide};

use crate::components::{Asteroid, FromPlayer, Laser, LaserDespawnTimer, SpriteSize, Player};

pub fn player_laser_hit_asteroid_system(
    mut commands: Commands,
    laser_query: Query<
        (Entity, &Transform, &SpriteSize, &LaserDespawnTimer),
        (With<Laser>, With<FromPlayer>),
    >,
    asteroid_query: Query<(Entity, &Transform, &SpriteSize), With<Asteroid>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // Iterate through player lasers
    for (laser_entity, laser_tf, laser_size, despawn_timer) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) || despawn_timer.0.just_finished() {
            continue;
        }

        let laser_scale = laser_tf.scale.xy();

        // Iterate trough asteroids
        for (asteroid_entity, asteroid_tf, asteroid_size) in asteroid_query.iter() {
            if despawned_entities.contains(&asteroid_entity)
                || despawned_entities.contains(&laser_entity)
                || despawn_timer.0.just_finished()
            {
                continue;
            }

            let asteroid_scale = asteroid_tf.scale.xy();

            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                asteroid_tf.translation,
                asteroid_size.0 * asteroid_scale,
            );

            // Check for collision
            if collision.is_some() {
                // Remove the asteroid
                commands.entity(asteroid_entity).despawn();
                despawned_entities.insert(asteroid_entity);

                // Remove the player laser
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);
            }
        }
    }
}

pub fn player_hit_asteroid_system(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform, &SpriteSize), With<Player>>,
    asteroid_query: Query<(Entity, &Transform, &SpriteSize), With<Asteroid>>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // Iterate through player
    for (player_entity, player_tf, player_size) in player_query.iter() {
        if despawned_entities.contains(&player_entity) {
            continue;
        }

        let player_scale = player_tf.scale.xy();

        // Iterate trough asteroids
        for (asteroid_entity, asteroid_tf, asteroid_size) in asteroid_query.iter() {
            if despawned_entities.contains(&asteroid_entity)
                || despawned_entities.contains(&player_entity)
            {
                continue;
            }

            let asteroid_scale = asteroid_tf.scale.xy();

            let collision = collide(
                player_tf.translation,
                player_size.0 * player_scale,
                asteroid_tf.translation,
                asteroid_size.0 * asteroid_scale,
            );

            // Check for collision
            if collision.is_some() {
                // Remove the asteroid
                commands.entity(asteroid_entity).despawn();
                despawned_entities.insert(asteroid_entity);

                // Remove the player
                commands.entity(player_entity).despawn();
                despawned_entities.insert(player_entity);
            }
        }
    }
}