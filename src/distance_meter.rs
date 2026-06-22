use crate::config;

pub struct DistanceMeter {
    pub distance: u32,
    pub high_score: u32,
    pub achievement: bool,
    flash_timer: f64,
    flash_iterations: u32,
    max_score: u32,
}

impl DistanceMeter {
    pub fn new() -> Self {
        Self {
            distance: 0,
            high_score: 0,
            achievement: false,
            flash_timer: 0.0,
            flash_iterations: 0,
            max_score: 99_999,
        }
    }

    pub fn update(&mut self, delta_ms: f64, distance_ran: f64) {
        let distance = self.get_actual_distance(distance_ran);
        if distance > 0 && distance % config::DISTANCE_ACHIEVEMENT == 0 && distance != self.distance
        {
            self.achievement = true;
            self.flash_timer = 0.0;
            self.flash_iterations = 0;
        }
        self.distance = distance.min(self.max_score);

        if self.achievement {
            self.flash_timer += delta_ms;
            if self.flash_timer >= config::DISTANCE_FLASH_DURATION {
                self.flash_timer = 0.0;
                self.flash_iterations += 1;
                if self.flash_iterations >= config::DISTANCE_FLASH_ITERATIONS {
                    self.achievement = false;
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.distance = 0;
        self.achievement = false;
        self.flash_timer = 0.0;
        self.flash_iterations = 0;
    }

    pub fn commit_high_score(&mut self) {
        self.high_score = self.high_score.max(self.distance);
    }

    pub fn distance_string(&self) -> String {
        format!(
            "{:0width$}",
            self.distance,
            width = config::DISTANCE_MAX_UNITS
        )
    }

    pub fn high_score_string(&self) -> String {
        format!(
            "HI {:0width$}",
            self.high_score,
            width = config::DISTANCE_MAX_UNITS
        )
    }

    fn get_actual_distance(&self, distance_ran: f64) -> u32 {
        (distance_ran * config::DISTANCE_COEFFICIENT).round() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_score_and_high_score() {
        let mut meter = DistanceMeter::new();
        meter.update(16.0, 1234.0);
        meter.commit_high_score();
        assert_eq!(meter.distance_string().len(), 5);
        assert!(meter.high_score_string().starts_with("HI "));
    }

    #[test]
    fn triggers_achievement_flash() {
        let mut meter = DistanceMeter::new();
        meter.update(16.0, 4000.0);
        assert!(meter.achievement);
    }
}
