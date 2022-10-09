use std::f32::consts::PI;

// asset constants
pub const PLAYER_SPRITE: &str = "playerShip2_blue.png";
pub const PLAYER_LASER_SPRITE: &str = "laserBlue01.png";
pub const BIG_ASTEROID_SPRITE: &str = "meteorBrown_big1.png";
pub const MED_ASTEROID_SPRITE: &str = "meteorBrown_med1.png";
pub const SMALL_ASTEROID_SPRITE: &str = "meteorBrown_small1.png";

pub const PLAYER_SIZE: (f32, f32) = (112., 75.);
pub const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
pub const BIG_ASTEROID_SIZE: (f32, f32) = (101., 84.);
pub const MED_ASTEROID_SIZE: (f32, f32) = (43., 43.);
pub const SMALL_ASTEROID_SIZE: (f32, f32) = (28., 28.);

pub const SPRITE_SCALE: f32 = 0.5;
pub const ASTEROID_SCALE: f32 = 1.5;

// game constants
pub const TIME_STEP: f32 = 1. / 60.;

// player constants
pub const PLAYER_ROTATION_SPEED: f32 = 5.0 * 2.0 * PI / 360.;
pub const PLAYER_MAX_SPEED: f32 = 7.0;
pub const PLAYER_ACCELERATION: f32 = 0.2;
pub const PLAYER_DECELERATION: f32 = 0.01;

// projectile constants
pub const PLAYER_LASER_SPEED: f32 = 13.0;

// asteroids' constants
pub const ASTEROID_ROTATION_SPEED: f32 = 1.4 / 360.;
