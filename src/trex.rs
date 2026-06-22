use rand::Rng;

use crate::collision::CollisionBox;
use crate::config;

const DUCKING_COLLISION_BOXES: &[CollisionBox] = &[CollisionBox::new(1.0, 18.0, 55.0, 25.0)];

const RUNNING_COLLISION_BOXES: &[CollisionBox] = &[
    CollisionBox::new(22.0, 0.0, 17.0, 16.0),
    CollisionBox::new(1.0, 18.0, 30.0, 9.0),
    CollisionBox::new(10.0, 35.0, 14.0, 8.0),
    CollisionBox::new(1.0, 24.0, 29.0, 5.0),
    CollisionBox::new(5.0, 30.0, 21.0, 4.0),
    CollisionBox::new(9.0, 34.0, 15.0, 4.0),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrexStatus {
    Crashed,
    Ducking,
    Jumping,
    Running,
    Waiting,
}

pub struct Trex {
    pub x_pos: f64,
    pub y_pos: f64,
    pub ground_y_pos: f64,
    pub current_frame: usize,
    pub blink_count: u32,
    pub status: TrexStatus,
    pub jumping: bool,
    pub ducking: bool,
    pub jump_velocity: f64,
    pub reached_min_height: bool,
    pub speed_drop: bool,
    pub jump_count: u32,
    pub playing_intro: bool,
    timer: f64,
    blink_delay: f64,
    anim_start_time: f64,
    min_jump_height: f64,
}

impl Trex {
    pub fn new() -> Self {
        let ground_y_pos = config::DEFAULT_HEIGHT - config::TREX_HEIGHT - config::BOTTOM_PAD;
        let mut trex = Self {
            x_pos: 0.0,
            y_pos: ground_y_pos,
            ground_y_pos,
            current_frame: 0,
            blink_count: 0,
            status: TrexStatus::Waiting,
            jumping: false,
            ducking: false,
            jump_velocity: 0.0,
            reached_min_height: false,
            speed_drop: false,
            jump_count: 0,
            playing_intro: false,
            timer: 0.0,
            blink_delay: 0.0,
            anim_start_time: 0.0,
            min_jump_height: ground_y_pos - config::TREX_MIN_JUMP_HEIGHT,
        };
        trex.set_blink_delay();
        trex
    }

    pub fn update(&mut self, delta_ms: f64, time_ms: f64, status: Option<TrexStatus>) {
        self.timer += delta_ms;
        if let Some(status) = status {
            self.status = status;
            self.current_frame = 0;
            if status == TrexStatus::Waiting {
                self.anim_start_time = time_ms;
                self.set_blink_delay();
            }
        }

        if self.playing_intro && self.x_pos < config::TREX_START_X_POS {
            self.x_pos +=
                ((config::TREX_START_X_POS / config::TREX_INTRO_DURATION) * delta_ms).round();
            if self.x_pos >= config::TREX_START_X_POS {
                self.playing_intro = false;
            }
        }

        if self.status == TrexStatus::Waiting {
            self.blink(time_ms);
        } else if self.timer >= self.ms_per_frame() {
            self.current_frame = if self.current_frame == self.frame_count() - 1 {
                0
            } else {
                self.current_frame + 1
            };
            self.timer = 0.0;
        }

        if self.speed_drop && (self.y_pos - self.ground_y_pos).abs() < f64::EPSILON {
            self.speed_drop = false;
            self.set_duck(true);
        }
    }

    pub fn start_jump(&mut self, speed: f64, time_ms: f64) {
        if !self.jumping {
            self.update(0.0, time_ms, Some(TrexStatus::Jumping));
            self.jump_velocity = config::TREX_INITIAL_JUMP_VELOCITY - speed / 10.0;
            self.jumping = true;
            self.reached_min_height = false;
            self.speed_drop = false;
            self.ducking = false;
        }
    }

    pub fn end_jump(&mut self) {
        if self.reached_min_height && self.jump_velocity < config::TREX_DROP_VELOCITY {
            self.jump_velocity = config::TREX_DROP_VELOCITY;
        }
    }

    pub fn update_jump(&mut self, delta_ms: f64, speed: f64, time_ms: f64) {
        let frames_elapsed = delta_ms / self.ms_per_frame();
        if self.speed_drop {
            self.y_pos +=
                (self.jump_velocity * config::TREX_SPEED_DROP_COEFFICIENT * frames_elapsed).round();
        } else {
            self.y_pos += (self.jump_velocity * frames_elapsed).round();
        }

        self.jump_velocity += config::TREX_GRAVITY * frames_elapsed;

        if self.y_pos < self.min_jump_height || self.speed_drop {
            self.reached_min_height = true;
        }

        if self.y_pos < config::TREX_MAX_JUMP_HEIGHT || self.speed_drop {
            self.end_jump();
        }

        if self.y_pos > self.ground_y_pos {
            self.reset(time_ms);
            self.jump_count += 1;
        }

        if self.jumping {
            let _ = speed;
            self.update(delta_ms, time_ms, None);
        }
    }

    pub fn set_speed_drop(&mut self) {
        self.speed_drop = true;
        self.jump_velocity = 1.0;
    }

    pub fn set_duck(&mut self, is_ducking: bool) {
        if is_ducking && self.status != TrexStatus::Ducking {
            self.status = TrexStatus::Ducking;
            self.current_frame = 0;
            self.ducking = true;
        } else if !is_ducking && self.status == TrexStatus::Ducking {
            self.status = TrexStatus::Running;
            self.current_frame = 0;
            self.ducking = false;
        }
    }

    pub fn reset(&mut self, _time_ms: f64) {
        self.y_pos = self.ground_y_pos;
        self.jump_velocity = 0.0;
        self.jumping = false;
        self.ducking = false;
        self.status = TrexStatus::Running;
        self.current_frame = 0;
        self.speed_drop = false;
        self.jump_count = 0;
    }

    pub fn crash(&mut self) {
        self.status = TrexStatus::Crashed;
        self.current_frame = 0;
    }

    pub fn collision_boxes(&self) -> &'static [CollisionBox] {
        if self.ducking {
            DUCKING_COLLISION_BOXES
        } else {
            RUNNING_COLLISION_BOXES
        }
    }

    pub fn sprite_key(&self) -> &'static str {
        match self.status {
            TrexStatus::Waiting => {
                if self.current_frame == 1 {
                    "trex_blink"
                } else {
                    "trex"
                }
            }
            TrexStatus::Crashed => "trex_crashed",
            TrexStatus::Ducking => {
                if self.current_frame == 0 {
                    "trex_duck_1"
                } else {
                    "trex_duck_2"
                }
            }
            TrexStatus::Jumping => "trex",
            TrexStatus::Running => {
                if self.current_frame == 0 {
                    "trex_run_1"
                } else {
                    "trex_run_2"
                }
            }
        }
    }

    fn set_blink_delay(&mut self) {
        self.blink_delay = rand::thread_rng()
            .gen_range(1.0..=config::TREX_BLINK_TIMING)
            .ceil();
    }

    fn blink(&mut self, time_ms: f64) {
        let delta_time = time_ms - self.anim_start_time;
        if delta_time >= self.blink_delay {
            if self.timer >= self.ms_per_frame() {
                self.current_frame = if self.current_frame == 0 { 1 } else { 0 };
                self.timer = 0.0;
            }
            if self.current_frame == 1 {
                self.set_blink_delay();
                self.anim_start_time = time_ms;
                self.blink_count += 1;
            }
        }
    }

    fn ms_per_frame(&self) -> f64 {
        match self.status {
            TrexStatus::Waiting => 1000.0 / 3.0,
            TrexStatus::Running => 1000.0 / 12.0,
            TrexStatus::Ducking => 1000.0 / 8.0,
            TrexStatus::Crashed | TrexStatus::Jumping => 1000.0 / 60.0,
        }
    }

    fn frame_count(&self) -> usize {
        match self.status {
            TrexStatus::Running | TrexStatus::Ducking | TrexStatus::Waiting => 2,
            TrexStatus::Crashed | TrexStatus::Jumping => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_lifecycle_returns_to_running() {
        let mut trex = Trex::new();
        trex.start_jump(config::SPEED, 0.0);
        for i in 0..80 {
            trex.update_jump(16.67, config::SPEED, i as f64 * 16.67);
        }
        assert_eq!(trex.status, TrexStatus::Running);
        assert_eq!(trex.y_pos, trex.ground_y_pos);
        assert!(!trex.jumping);
    }

    #[test]
    fn speed_drop_sets_duck_on_landing() {
        let mut trex = Trex::new();
        trex.start_jump(config::SPEED, 0.0);
        trex.set_speed_drop();
        for i in 0..20 {
            trex.update_jump(16.67, config::SPEED, i as f64 * 16.67);
        }
        trex.set_duck(true);
        assert!(trex.ducking);
    }
}
