use bevy::prelude::*;
use rand::prelude::*;

use crate::{resources::{GameTextures, WinSize}, constants::{BIG_ASTEROID_SIZE, ASTEROID_SCALE}, components::{Asteroid, SpriteSize, Velocity, Position}};

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
            rng.gen_range(dist_range.clone())
        ),

        (
            120. + rng.gen_range(angle_offset_range.clone()),
            rng.gen_range(dist_range.clone())
        ),

        (
            240. + rng.gen_range(angle_offset_range.clone()),
            rng.gen_range(dist_range.clone()),
        ),
    ];

    for (angle, dist) in polar.iter() {
        let (x, y) = angle.to_radians().sin_cos();
        let position = Vec2::new(x * dist, y * dist);

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_textures.big_asteroid.clone(),
                transform: Transform {
                    translation: Vec3::new(position.x, position.y, 10.),
                    scale: Vec3::new(ASTEROID_SCALE, ASTEROID_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Name::new("Big Asteroid"))
            .insert(Asteroid { size: 4 })
            .insert(SpriteSize::from(BIG_ASTEROID_SIZE))
            .insert(Velocity(Vec2::splat(0.0)))
            .insert(Position(Vec2::new(position.x, position.y)));
    }
}