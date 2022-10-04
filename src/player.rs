use bevy::prelude::*;

use crate::{
    components::{Player, Velocity, Position},
    constants::{PLAYER_ROTATION_SPEED, PLAYER_ACCELERATION, PLAYER_MAX_SPEED, PLAYER_DECELERATION}, resources::WinSize,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(player_keyboard_event_system)
            .add_system(player_movement_system)
            .add_system(sync_player_transform_system.after(player_movement_system));
    }
}

// player systems
fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Velocity)>,
) {
    for (mut player, mut velocity) in query.iter_mut() {
        if kb.pressed(KeyCode::Left) {
            player.rotation_angle += PLAYER_ROTATION_SPEED;
        } else if kb.pressed(KeyCode::Right) {
            player.rotation_angle -= PLAYER_ROTATION_SPEED;
        }

        if kb.pressed(KeyCode::Up) {
            velocity.0 += player.direction() * PLAYER_ACCELERATION;

            if velocity.0.length() > PLAYER_MAX_SPEED  {
                velocity.0 = velocity.0.normalize_or_zero() * PLAYER_MAX_SPEED;
            }
        } else if !kb.pressed(KeyCode::Up)  {
            velocity.0 *= 1.0 - PLAYER_DECELERATION;
        }
    }
}

fn player_movement_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Velocity, &mut Position, &Player, &mut Transform)>,
    win_size: Res<WinSize>,
) {
    // values containing each corener of the screen
    let right_side = win_size.w / 2.0;
    let left_side = -right_side;
    let top =  win_size.h / 2.0;
    let bottom = -top;

    for (entity, velocity, mut position, player, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
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

        transform.rotation = Quat::from_rotation_z(player.rotation_angle);
        position.0 = new_position;
    }
}

fn sync_player_transform_system(
    mut query: Query<(&Position, &mut Transform), With<Player>>,
) {
    for (position, mut transform) in query.get_single_mut() {
        transform.translation = Vec3::new(
            position.0.x,
            position.0.y,
            transform.translation.z,
        );
    }
}