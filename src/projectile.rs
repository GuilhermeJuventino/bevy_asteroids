use bevy::prelude::*;

use crate::{components::{Velocity, Position, Laser, LaserDespawnTimer}, resources::WinSize};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(projectile_movement_system)
            .add_system(projectile_despawn_system)
            .add_system(sync_laser_transform_system.after(projectile_movement_system));
    }
}


fn projectile_movement_system(
    mut query: Query<(&Velocity, &mut Position, &Transform)>,
    win_size: Res<WinSize>,
) {
    // values containing each corener of the screen
    let right_side = win_size.w / 2.0;
    let left_side = -right_side;
    let top =  win_size.h / 2.0;
    let bottom = -top;

    for (velocity, mut position, transform) in query.iter_mut() {
        let mut new_position = position.0 + velocity.0;
        let half_scale = transform.scale.max_element();

        // screen wrapping
        if new_position.x > right_side + half_scale {
            new_position.x = left_side - half_scale;
        } else if new_position.x < left_side - half_scale {
            new_position.x = right_side + half_scale;
        }

        if new_position.y > top + half_scale {
            new_position.y = bottom - half_scale;
        } else if new_position.y < bottom - half_scale {
            new_position.y = top + half_scale;
        }

        position.0 = new_position;
    }
}

fn sync_laser_transform_system(
    mut query: Query<(&Position, &mut Transform), With<Laser>>,
) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            position.0.x,
            position.0.y,
            transform.translation.z,
        );
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