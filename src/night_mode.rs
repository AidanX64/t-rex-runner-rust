use rand::Rng;

use crate::config;

#[derive(Clone, Copy)]
pub struct Star {
    pub x: f64,
    pub y: f64,
}

pub struct NightMode {
    pub x_pos: f64,
    pub y_pos: f64,
    pub current_phase: usize,
    pub opacity: f64,
    pub stars: Vec<Star>,
    pub draw_stars: bool,
    container_width: f64,
}

impl NightMode {
    pub fn new(container_width: f64) -> Self {
        let mut mode = Self {
            x_pos: container_width - 50.0,
            y_pos: 30.0,
            current_phase: 0,
            opacity: 0.0,
            stars: Vec::new(),
            draw_stars: false,
            container_width,
        };
        mode.place_stars();
        mode
    }

    pub fn update(&mut self, activated: bool) {
        if activated && self.opacity == 0.0 {
            self.current_phase += 1;
            if self.current_phase >= 7 {
                self.current_phase = 0;
            }
        }

        if activated && (self.opacity - 1.0).abs() > f64::EPSILON {
            self.opacity = (self.opacity + config::NIGHT_FADE_SPEED).min(1.0);
        } else if !activated && self.opacity > 0.0 {
            self.opacity = (self.opacity - config::NIGHT_FADE_SPEED).max(0.0);
        }

        if self.opacity > 0.0 {
            self.x_pos = self.update_x_pos(self.x_pos, config::NIGHT_MOON_SPEED);
            self.draw_stars = true;
            for star in &mut self.stars {
                star.x =
                    Self::update_star_x_pos(star.x, config::NIGHT_STAR_SPEED, self.container_width);
            }
        } else {
            self.draw_stars = false;
        }
    }

    pub fn reset(&mut self) {
        self.opacity = 0.0;
        self.current_phase = 0;
        self.x_pos = self.container_width - 50.0;
        self.place_stars();
    }

    pub fn resize(&mut self, width: f64) {
        self.container_width = width;
        self.x_pos = self.x_pos.min(width - config::NIGHT_WIDTH);
    }

    fn update_x_pos(&self, current_pos: f64, speed: f64) -> f64 {
        if current_pos < -config::NIGHT_WIDTH {
            self.container_width
        } else {
            current_pos - speed
        }
    }

    fn update_star_x_pos(current_pos: f64, speed: f64, container_width: f64) -> f64 {
        if current_pos < -config::NIGHT_STAR_SIZE {
            container_width
        } else {
            current_pos - speed
        }
    }

    fn place_stars(&mut self) {
        self.stars.clear();
        let mut rng = rand::thread_rng();
        let segment = self.container_width / config::NIGHT_NUM_STARS as f64;
        for i in 0..config::NIGHT_NUM_STARS {
            self.stars.push(Star {
                x: rng.gen_range((segment * i as f64)..(segment * (i + 1) as f64)),
                y: rng.gen_range(0..=config::NIGHT_STAR_MAX_Y) as f64,
            });
        }
    }
}
