use bevy::prelude::*;
use rand::prelude::*;

use crate::{resources::{GameTextures, WinSize}, constants::{BIG_ASTEROID_SIZE, ASTEROID_SCALE}, components::{Asteroid, SpriteSize, Velocity, Position}};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(asteroid_spawning_system);
    }
}


fn asteroid_spawning_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let mut rng = thread_rng();
    for _ in 0..3 {
        let x = rng.gen_range(-1..1) as f32;
        let y = rng.gen_range(-1..1) as f32;

        let position = Vec2::new(x, y).normalize() * win_size.w.min(win_size.h) / 2.;

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