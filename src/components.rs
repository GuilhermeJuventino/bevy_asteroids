use std::f32::consts::PI;

use bevy::prelude::*;

// common components
#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct RotationAngle(pub f32);

/*#[derive(Component)]
pub struct Rotate {
    pub z: f32,
}*/

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct Laser {
    pub starting_position: Vec2,
}

#[derive(Component)]
pub struct LaserDespawnTimer(pub Timer);

impl Default for LaserDespawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.9, false))
    }
}

// player components
#[derive(Component)]
pub struct Player;

impl Player {
    pub fn direction(&self, rotation_angle: f32) -> Vec2 {
        let (y, x) = (rotation_angle + PI / 2.0).sin_cos();

        Vec2::new(x, y)
    }
}

#[derive(Component)]
pub struct FromPlayer;

#[derive(Component)]
pub struct PlayerLaserCooldown(pub Timer);

impl Default for PlayerLaserCooldown {
    fn default() -> Self {
        Self(Timer::from_seconds(0.24, false))
    }
}

// asteroid components
#[derive(Component)]
pub struct Asteroid {
    pub size: i32,
    pub rotation_speed: f32,
}
