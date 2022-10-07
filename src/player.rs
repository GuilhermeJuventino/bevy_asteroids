use bevy::prelude::*;

use crate::{
    components::{
        FromPlayer, Laser, LaserDespawnTimer, Player, PlayerLaserCooldown, Position, RotationAngle,
        SpriteSize, Velocity,
    },
    constants::{
        PLAYER_ACCELERATION, PLAYER_DECELERATION, PLAYER_LASER_SIZE, PLAYER_LASER_SPEED,
        PLAYER_MAX_SPEED, PLAYER_ROTATION_SPEED, SPRITE_SCALE,
    },
    resources::GameTextures,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_keyboard_event_system)
            .add_system(player_shoot_projectile_system);
    }
}

// player systems
fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Velocity, &mut RotationAngle)>,
) {
    for (player, mut velocity, mut rotation_angle) in query.iter_mut() {
        // rotate the player ship
        if kb.pressed(KeyCode::Left) {
            rotation_angle.0 += PLAYER_ROTATION_SPEED;
        } else if kb.pressed(KeyCode::Right) {
            rotation_angle.0 -= PLAYER_ROTATION_SPEED;
        }

        // accelerate the ship towards the direction it's currently facing
        if kb.pressed(KeyCode::Up) {
            velocity.0 += player.direction(rotation_angle.0) * PLAYER_ACCELERATION;

            if velocity.0.length() > PLAYER_MAX_SPEED {
                velocity.0 = velocity.0.normalize_or_zero() * PLAYER_MAX_SPEED;
            }
        } else if !kb.pressed(KeyCode::Up) {
            velocity.0 *= 1.0 - PLAYER_DECELERATION;
        }
    }
}

fn player_shoot_projectile_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    time: Res<Time>,
    mut query: Query<(&Player, &RotationAngle, &Position, &mut PlayerLaserCooldown)>,
) {
    for (player, rotation_angle, position, mut laser_cooldown) in query.iter_mut() {
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
                            rotation: Quat::from_rotation_z(rotation_angle.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Name::new("Player laser"))
                    .insert(Laser {
                        starting_position: position.0.clone(),
                    })
                    .insert(LaserDespawnTimer::default())
                    .insert(FromPlayer)
                    .insert(SpriteSize::from(PLAYER_LASER_SIZE))
                    .insert(Velocity(
                        player.direction(rotation_angle.0).normalize() * PLAYER_LASER_SPEED,
                    ))
                    .insert(Position(position.0.clone()));

                has_fired = true;
            }

            if has_fired {
                laser_cooldown.0.reset();
            }
        }
    }
}
