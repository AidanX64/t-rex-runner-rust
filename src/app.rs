use crate::collision::check_for_collision;
use crate::config;
use crate::distance_meter::DistanceMeter;
use crate::game_over_panel::GameOverPanel;
use crate::horizon::Horizon;
use crate::input::InputAction;
use crate::trex::{Trex, TrexStatus};

pub struct App {
    pub trex: Trex,
    pub horizon: Horizon,
    pub distance_meter: DistanceMeter,
    pub game_over_panel: GameOverPanel,
    pub current_speed: f64,
    pub distance_ran: f64,
    pub playing: bool,
    pub crashed: bool,
    pub paused: bool,
    pub activated: bool,
    pub quit: bool,
    pub width: u16,
    pub height: u16,
    time_ms: f64,
    running_time: f64,
    game_over_time: f64,
    invert_timer: f64,
    invert_triggered: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            trex: Trex::new(),
            horizon: Horizon::new(),
            distance_meter: DistanceMeter::new(),
            game_over_panel: GameOverPanel::new(),
            current_speed: config::SPEED,
            distance_ran: 0.0,
            playing: false,
            crashed: false,
            paused: false,
            activated: false,
            quit: false,
            width: 0,
            height: 0,
            time_ms: 0.0,
            running_time: 0.0,
            game_over_time: 0.0,
            invert_timer: 0.0,
            invert_triggered: false,
        }
    }

    pub fn update(&mut self, delta_ms: f64) {
        self.time_ms += delta_ms;
        if self.paused {
            return;
        }

        if !self.playing {
            self.trex.update(delta_ms, self.time_ms, None);
            return;
        }

        if self.crashed {
            self.game_over_time += delta_ms;
            return;
        }

        self.running_time += delta_ms;
        let update_obstacles =
            !self.trex.playing_intro || self.trex.x_pos >= config::TREX_START_X_POS;
        self.horizon.update(
            delta_ms,
            self.current_speed,
            update_obstacles,
            self.invert_triggered,
        );

        if self.trex.jumping {
            self.trex
                .update_jump(delta_ms, self.current_speed, self.time_ms);
        } else {
            self.trex.update(delta_ms, self.time_ms, None);
        }

        if self.current_speed < config::MAX_SPEED {
            self.current_speed += config::ACCELERATION;
        }

        self.distance_ran += self.current_speed * delta_ms / config::FPS;
        self.distance_meter.update(delta_ms, self.distance_ran);
        self.update_night_mode(delta_ms);

        if let Some(obstacle) = self.horizon.obstacles.first() {
            if check_for_collision(obstacle, &self.trex).is_some() {
                self.crash();
            }
        }
    }

    pub fn handle_action(&mut self, action: InputAction) {
        match action {
            InputAction::JumpPressed => {
                if self.crashed {
                    self.restart_if_ready();
                } else {
                    if !self.playing {
                        self.start_game();
                    }
                    if !self.trex.jumping && !self.trex.ducking {
                        self.trex.start_jump(self.current_speed, self.time_ms);
                    }
                }
            }
            InputAction::JumpReleased => {
                if self.trex.jumping {
                    self.trex.end_jump();
                }
            }
            InputAction::DuckPressed => {
                if self.trex.jumping {
                    self.trex.set_speed_drop();
                } else if self.playing && !self.crashed {
                    self.trex.set_duck(true);
                }
            }
            InputAction::DuckReleased => {
                self.trex.set_duck(false);
            }
            InputAction::Restart => {
                if self.crashed {
                    self.restart_if_ready();
                } else if !self.playing {
                    self.start_game();
                }
            }
            InputAction::Pause => self.paused = true,
            InputAction::Resume => self.paused = false,
            InputAction::Quit => self.quit = true,
            InputAction::None => {}
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.horizon.resize(config::DEFAULT_WIDTH);
    }

    pub fn can_restart(&self) -> bool {
        self.game_over_time >= config::GAMEOVER_CLEAR_TIME
    }

    fn start_game(&mut self) {
        self.playing = true;
        self.activated = true;
        self.trex.playing_intro = true;
        self.trex
            .update(0.0, self.time_ms, Some(TrexStatus::Running));
    }

    fn crash(&mut self) {
        self.crashed = true;
        self.playing = true;
        self.game_over_time = 0.0;
        self.trex.crash();
        self.distance_meter.commit_high_score();
        self.game_over_panel.show();
    }

    fn restart_if_ready(&mut self) {
        if self.can_restart() {
            self.restart();
        }
    }

    fn restart(&mut self) {
        let high_score = self.distance_meter.high_score;
        self.trex = Trex::new();
        self.horizon.reset();
        self.distance_meter.reset();
        self.distance_meter.high_score = high_score;
        self.game_over_panel.hide();
        self.current_speed = config::SPEED;
        self.distance_ran = 0.0;
        self.playing = false;
        self.crashed = false;
        self.paused = false;
        self.running_time = 0.0;
        self.game_over_time = 0.0;
        self.invert_timer = 0.0;
        self.invert_triggered = false;
    }

    fn update_night_mode(&mut self, delta_ms: f64) {
        let distance = self.distance_meter.distance as f64;
        if distance > 0.0 && distance % config::INVERT_DISTANCE == 0.0 {
            self.invert_triggered = true;
            self.invert_timer = 0.0;
        }

        if self.invert_triggered {
            self.invert_timer += delta_ms;
            if self.invert_timer > config::INVERT_FADE_DURATION {
                self.invert_triggered = false;
                self.invert_timer = 0.0;
            }
        }
    }
}
