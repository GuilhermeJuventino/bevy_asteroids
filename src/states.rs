use bevy::prelude::*;

use crate::{
    asteroids::asteroid_spawning_system,
    components::{Position, RotationAngle, Velocity},
    resources::{GameStates, WinSize},
};

pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameStates::InGame).with_system(asteroid_spawning_system),
        )
        .add_system_set(
            SystemSet::on_update(GameStates::InGame)
                .with_system(movement_system)
                .with_system(sync_transform_system.after(movement_system))
                .with_system(sync_rotation_system),
        )
        .add_system_set(SystemSet::on_exit(GameStates::InGame));
    }
}

// Systems that run when entering InGame state

// Systems that run when updating InGame State
fn movement_system(
    mut query: Query<(&Velocity, &mut Position, &Transform)>,
    win_size: Res<WinSize>,
) {
    // values containing each corener of the screen
    let right_side = win_size.w / 2.0;
    let left_side = -right_side;
    let top = win_size.h / 2.0;
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

fn sync_transform_system(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(position.0.x, position.0.y, transform.translation.z);
    }
}

fn sync_rotation_system(mut query: Query<(&RotationAngle, &mut Transform)>) {
    for (rotation_angle, mut transform) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(rotation_angle.0);
    }
}
