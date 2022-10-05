use bevy::prelude::*;

use crate::{
    components::{Player, Velocity, Position, PlayerLaserCooldown, Laser, SpriteSize, FromPlayer, LaserDespawnTimer},
    constants::{PLAYER_ROTATION_SPEED, PLAYER_ACCELERATION, PLAYER_MAX_SPEED, PLAYER_DECELERATION, SPRITE_SCALE, PLAYER_LASER_SIZE, PLAYER_LASER_SPEED}, resources::{WinSize, GameTextures},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(player_keyboard_event_system)
            .add_system(player_movement_system)
            .add_system(sync_player_transform_system.after(player_movement_system))
            .add_system(player_shoot_projectile_system);
    }
}

// player systems
fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Velocity)>,
) {
    for (mut player, mut velocity) in query.iter_mut() {
        // rotate the player ship
        if kb.pressed(KeyCode::Left) {
            player.rotation_angle += PLAYER_ROTATION_SPEED;
        } else if kb.pressed(KeyCode::Right) {
            player.rotation_angle -= PLAYER_ROTATION_SPEED;
        }
        
        // accelerate the ship towards the direction it's currently facing
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
    mut query: Query<(&Velocity, &mut Position, &Player, &mut Transform)>,
    win_size: Res<WinSize>,
) {
    // values containing each corener of the screen
    let right_side = win_size.w / 2.0;
    let left_side = -right_side;
    let top =  win_size.h / 2.0;
    let bottom = -top;

    for (velocity, mut position, player, mut transform) in query.iter_mut() {
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

fn player_shoot_projectile_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    time: Res<Time>,
    mut query: Query<(&Player, &Position, &mut PlayerLaserCooldown)>,
) {
    for (player, position, mut laser_cooldown) in query.iter_mut() {
        laser_cooldown.0.tick(time.delta());
        let mut has_fired = false;

        if laser_cooldown.0.finished() {
            if kb.pressed(KeyCode::Z) {
                commands
                    .spawn_bundle(SpriteBundle {
                        texture: game_textures.player_laser.clone(),
                        transform: Transform {
                            translation: Vec3::new(position.0.x, position.0.y, 5.),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                            rotation: Quat::from_rotation_z(player.rotation_angle),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Name::new("Player laser"))
                    .insert(Laser {starting_position: position.0.clone()})
                    .insert(LaserDespawnTimer::default())
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Velocity(
                        player.direction().normalize() * PLAYER_LASER_SPEED)
                    )
                    .insert(Position(position.0.clone()));

                    has_fired = true;
            }

            if has_fired {
                laser_cooldown.0.reset();
            }
        }
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