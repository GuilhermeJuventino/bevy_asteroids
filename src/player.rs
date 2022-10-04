use bevy::prelude::*;

use crate::{
    components::{Player, Velocity, Position},
    constants::{PLAYER_ROTATION_SPEED, PLAYER_ACCELERATION, PLAYER_MAX_SPEED, PLAYER_DECELERATION},
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
        player.rotation_angle = if kb.pressed(KeyCode::Left) {
            PLAYER_ROTATION_SPEED
        } else if kb.pressed(KeyCode::Right) {
            -PLAYER_ROTATION_SPEED
        } else {
            0.
        };

        if kb.pressed(KeyCode::Up) {
            velocity.0 += player.direction() * PLAYER_ACCELERATION;
            println!("{}", velocity.0);
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
) {
    for (entity, velocity, mut position, player, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let mut new_position = position.0 + velocity.0;

        transform.rotate_z(player.rotation_angle);
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