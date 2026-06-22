use rand::Rng;

use crate::config;

pub struct Cloud {
    pub x_pos: f64,
    pub y_pos: f64,
    pub remove: bool,
    pub cloud_gap: f64,
}

impl Cloud {
    pub fn new(container_width: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x_pos: container_width,
            y_pos: rng.gen_range(config::MAX_SKY_LEVEL..=config::MIN_SKY_LEVEL) as f64,
            remove: false,
            cloud_gap: rng.gen_range(config::MIN_CLOUD_GAP..=config::MAX_CLOUD_GAP) as f64,
        }
    }

    pub fn update(&mut self, speed: f64) {
        if !self.remove {
            self.x_pos -= speed.ceil();
            if !self.is_visible() {
                self.remove = true;
            }
        }
    }

    pub fn is_visible(&self) -> bool {
        self.x_pos + config::CLOUD_WIDTH > 0.0
    }
}
