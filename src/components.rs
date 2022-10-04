use std::f32::consts::PI;

use bevy::prelude::*;

// common components
#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Rotate {
    pub z: f32,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

// player components
#[derive(Component)]
pub struct Player {
    pub rotation_angle: f32,
}

impl Player {
    pub fn direction(&self) -> Vec2 {
        let (y, x) = (self.rotation_angle + PI / 2.0).sin_cos();
        
        Vec2::new(x, y)
    }
}

// asteroid components
pub struct Asteroid;
