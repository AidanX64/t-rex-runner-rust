#![allow(dead_code)]

use std::time::Duration;

pub const FPS: f64 = 60.0;
pub const MS_PER_FRAME: Duration = Duration::from_millis(16);
pub const DEFAULT_WIDTH: f64 = 600.0;
pub const DEFAULT_HEIGHT: f64 = 150.0;

pub const ACCELERATION: f64 = 0.001;
pub const BG_CLOUD_SPEED: f64 = 0.2;
pub const BOTTOM_PAD: f64 = 10.0;
pub const CLEAR_TIME: f64 = 3000.0;
pub const CLOUD_FREQUENCY: f64 = 0.5;
pub const GAMEOVER_CLEAR_TIME: f64 = 750.0;
pub const GAP_COEFFICIENT: f64 = 0.6;
pub const INVERT_FADE_DURATION: f64 = 12000.0;
pub const INVERT_DISTANCE: f64 = 700.0;
pub const MAX_BLINK_COUNT: u32 = 3;
pub const MAX_CLOUDS: usize = 6;
pub const MAX_OBSTACLE_LENGTH: u32 = 3;
pub const MAX_OBSTACLE_DUPLICATION: usize = 2;
pub const MAX_SPEED: f64 = 13.0;
pub const MOBILE_SPEED_COEFFICIENT: f64 = 1.2;
pub const SPEED: f64 = 6.0;

pub const TREX_DROP_VELOCITY: f64 = -5.0;
pub const TREX_GRAVITY: f64 = 0.6;
pub const TREX_HEIGHT: f64 = 47.0;
pub const TREX_HEIGHT_DUCK: f64 = 25.0;
pub const TREX_INITIAL_JUMP_VELOCITY: f64 = -10.0;
pub const TREX_INTRO_DURATION: f64 = 1500.0;
pub const TREX_MAX_JUMP_HEIGHT: f64 = 30.0;
pub const TREX_MIN_JUMP_HEIGHT: f64 = 30.0;
pub const TREX_SPEED_DROP_COEFFICIENT: f64 = 3.0;
pub const TREX_START_X_POS: f64 = 50.0;
pub const TREX_WIDTH: f64 = 44.0;
pub const TREX_WIDTH_DUCK: f64 = 59.0;
pub const TREX_BLINK_TIMING: f64 = 7000.0;

pub const HORIZON_WIDTH: f64 = 600.0;
pub const HORIZON_HEIGHT: f64 = 12.0;
pub const HORIZON_YPOS: f64 = 127.0;
pub const HORIZON_BUMPY_THRESHOLD: f64 = 0.3;
pub const HORIZON_CLOUD_FREQUENCY: f64 = 0.5;

pub const CLOUD_WIDTH: f64 = 46.0;
pub const CLOUD_HEIGHT: f64 = 14.0;
pub const MAX_CLOUD_GAP: i32 = 400;
pub const MAX_SKY_LEVEL: i32 = 30;
pub const MIN_CLOUD_GAP: i32 = 100;
pub const MIN_SKY_LEVEL: i32 = 71;

pub const NIGHT_FADE_SPEED: f64 = 0.035;
pub const NIGHT_HEIGHT: f64 = 40.0;
pub const NIGHT_MOON_SPEED: f64 = 0.25;
pub const NIGHT_NUM_STARS: usize = 2;
pub const NIGHT_STAR_SIZE: f64 = 9.0;
pub const NIGHT_STAR_SPEED: f64 = 0.3;
pub const NIGHT_STAR_MAX_Y: i32 = 70;
pub const NIGHT_WIDTH: f64 = 20.0;

pub const DISTANCE_MAX_UNITS: usize = 5;
pub const DISTANCE_ACHIEVEMENT: u32 = 100;
pub const DISTANCE_COEFFICIENT: f64 = 0.025;
pub const DISTANCE_FLASH_DURATION: f64 = 1000.0 / 4.0;
pub const DISTANCE_FLASH_ITERATIONS: u32 = 3;
