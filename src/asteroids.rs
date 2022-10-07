use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    components::{Asteroid, Position, RotationAngle, SpriteSize, Velocity},
    constants::{ASTEROID_ROTATION_SPEED, ASTEROID_SCALE, BIG_ASTEROID_SIZE},
    resources::{GameTextures, WinSize},
};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(asteroid_rotation_system);
    }
}

pub fn asteroid_spawning_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let mut rng = thread_rng();
    let max_dist = win_size.w.min(win_size.h) / 2.;
    let min_dist = 320. as f32;
    let dist_range = min_dist..max_dist;
    let angle_offset_range = 0.0..100.0 as f32;

    // array with 3 evenly spaced angles
    let polar = vec![
        (
            0. + rng.gen_range(angle_offset_range.clone()),
            rng.gen_range(dist_range.clone()),
        ),
        (
            120. + rng.gen_range(angle_offset_range.clone()),
            rng.gen_range(dist_range.clone()),
        ),
        (
            240. + rng.gen_range(angle_offset_range.clone()),
            rng.gen_range(dist_range.clone()),
        ),
    ];

    for (angle, dist) in polar.iter() {
        // calculating coordinates to spawn the asteroids
        let (x, y) = angle.to_radians().sin_cos();
        let position = Vec2::new(x * dist, y * dist);

        // randomizing the starting rotation angle of the asteroids
        let randomized_rotation_angle = rng.gen_range(-1.0..1.0) as f32;

        // randomizing asteroid movement speed
        let asteroid_speed = Vec2::new(rng.gen_range(-2.0..2.0), rng.gen_range(-2.0..2.0));

        // randomizing asteroid rotation speed
        let asteroid_rotation_speed =
            rng.gen_range(-ASTEROID_ROTATION_SPEED..ASTEROID_ROTATION_SPEED);

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.big_asteroid.clone(),
                transform: Transform {
                    translation: Vec3::new(position.x, position.y, 10.),
                    rotation: Quat::from_rotation_z(randomized_rotation_angle),
                    scale: Vec3::new(ASTEROID_SCALE, ASTEROID_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Big Asteroid"))
            .insert(Asteroid {
                size: 4,
                rotation_speed: asteroid_rotation_speed,
            })
            .insert(SpriteSize::from(BIG_ASTEROID_SIZE))
            .insert(Velocity(Vec2::from(asteroid_speed)))
            .insert(Position(Vec2::new(position.x, position.y)))
            .insert(RotationAngle(randomized_rotation_angle));
    }
}

fn asteroid_rotation_system(mut query: Query<(&Asteroid, &mut RotationAngle)>) {
    for (asteroid, mut rotation_angle) in query.iter_mut() {
        rotation_angle.0 += asteroid.rotation_speed;
    }
}
