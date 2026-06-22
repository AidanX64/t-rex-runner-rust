use rand::Rng;

use crate::cloud::Cloud;
use crate::config;
use crate::night_mode::NightMode;
use crate::obstacle::{OBSTACLE_TYPES, Obstacle, ObstacleKind};

pub struct HorizonLine {
    pub x_pos: [f64; 2],
    pub source_x_pos: [f64; 2],
    bump_threshold: f64,
}

impl HorizonLine {
    pub fn new() -> Self {
        Self {
            x_pos: [0.0, config::HORIZON_WIDTH],
            source_x_pos: [0.0, config::HORIZON_WIDTH],
            bump_threshold: config::HORIZON_BUMPY_THRESHOLD,
        }
    }

    pub fn update(&mut self, delta_ms: f64, speed: f64) {
        let increment = (speed * (config::FPS / 1000.0) * delta_ms).floor();
        if self.x_pos[0] <= 0.0 {
            self.update_x_pos(0, increment);
        } else {
            self.update_x_pos(1, increment);
        }
    }

    pub fn reset(&mut self) {
        self.x_pos = [0.0, config::HORIZON_WIDTH];
    }

    fn update_x_pos(&mut self, pos: usize, increment: f64) {
        let line2 = if pos == 0 { 1 } else { 0 };
        self.x_pos[pos] -= increment;
        self.x_pos[line2] = self.x_pos[pos] + config::HORIZON_WIDTH;
        if self.x_pos[pos] <= -config::HORIZON_WIDTH {
            self.x_pos[pos] += config::HORIZON_WIDTH * 2.0;
            self.x_pos[line2] = self.x_pos[pos] - config::HORIZON_WIDTH;
            self.source_x_pos[pos] = self.get_random_type();
        }
    }

    fn get_random_type(&self) -> f64 {
        if rand::random::<f64>() > self.bump_threshold {
            config::HORIZON_WIDTH
        } else {
            0.0
        }
    }
}

pub struct Horizon {
    pub obstacles: Vec<Obstacle>,
    pub obstacle_history: Vec<ObstacleKind>,
    pub clouds: Vec<Cloud>,
    pub night_mode: NightMode,
    pub horizon_line: HorizonLine,
    pub dimensions_width: f64,
    gap_coefficient: f64,
    cloud_frequency: f64,
    cloud_speed: f64,
}

impl Horizon {
    pub fn new() -> Self {
        let mut horizon = Self {
            obstacles: Vec::new(),
            obstacle_history: Vec::new(),
            clouds: Vec::new(),
            night_mode: NightMode::new(config::DEFAULT_WIDTH),
            horizon_line: HorizonLine::new(),
            dimensions_width: config::DEFAULT_WIDTH,
            gap_coefficient: config::GAP_COEFFICIENT,
            cloud_frequency: config::HORIZON_CLOUD_FREQUENCY,
            cloud_speed: config::BG_CLOUD_SPEED,
        };
        horizon.add_cloud();
        horizon
    }

    pub fn update(
        &mut self,
        delta_ms: f64,
        current_speed: f64,
        update_obstacles: bool,
        show_night_mode: bool,
    ) {
        self.horizon_line.update(delta_ms, current_speed);
        self.night_mode.update(show_night_mode);
        self.update_clouds(delta_ms, current_speed);
        if update_obstacles {
            self.update_obstacles(delta_ms, current_speed);
        }
    }

    pub fn reset(&mut self) {
        self.obstacles.clear();
        self.horizon_line.reset();
        self.night_mode.reset();
    }

    pub fn resize(&mut self, width: f64) {
        self.dimensions_width = width;
        self.night_mode.resize(width);
    }

    fn update_clouds(&mut self, delta_ms: f64, speed: f64) {
        let cloud_speed = self.cloud_speed / 1000.0 * delta_ms * speed;
        for cloud in &mut self.clouds {
            cloud.update(cloud_speed);
        }

        if let Some(last_cloud) = self.clouds.last() {
            if self.clouds.len() < config::MAX_CLOUDS
                && (self.dimensions_width - last_cloud.x_pos) > last_cloud.cloud_gap
                && self.cloud_frequency > rand::random::<f64>()
            {
                self.add_cloud();
            }
        } else {
            self.add_cloud();
        }

        self.clouds.retain(|cloud| !cloud.remove);
    }

    fn update_obstacles(&mut self, delta_ms: f64, current_speed: f64) {
        for obstacle in &mut self.obstacles {
            obstacle.update(delta_ms, current_speed);
        }
        self.obstacles.retain(|obstacle| !obstacle.remove);

        if let Some(last_obstacle) = self.obstacles.last_mut() {
            if !last_obstacle.following_obstacle_created
                && last_obstacle.is_visible()
                && last_obstacle.x_pos + last_obstacle.width + last_obstacle.gap
                    < self.dimensions_width
            {
                last_obstacle.following_obstacle_created = true;
                self.add_new_obstacle(current_speed);
            }
        } else {
            self.add_new_obstacle(current_speed);
        }
    }

    fn add_new_obstacle(&mut self, current_speed: f64) {
        for _ in 0..10 {
            let index = rand::thread_rng().gen_range(0..OBSTACLE_TYPES.len());
            let obstacle_type = OBSTACLE_TYPES[index];
            if !self.duplicate_obstacle_check(obstacle_type.kind)
                && current_speed >= obstacle_type.min_speed
            {
                self.obstacles.push(Obstacle::new(
                    obstacle_type,
                    self.gap_coefficient,
                    current_speed,
                    obstacle_type.width,
                ));
                self.obstacle_history.insert(0, obstacle_type.kind);
                if self.obstacle_history.len() > config::MAX_OBSTACLE_DUPLICATION {
                    self.obstacle_history
                        .truncate(config::MAX_OBSTACLE_DUPLICATION);
                }
                return;
            }
        }
    }

    fn duplicate_obstacle_check(&self, next_obstacle_type: ObstacleKind) -> bool {
        let mut duplicate_count = 0;
        for obstacle_type in &self.obstacle_history {
            duplicate_count = if *obstacle_type == next_obstacle_type {
                duplicate_count + 1
            } else {
                0
            };
        }
        duplicate_count >= config::MAX_OBSTACLE_DUPLICATION
    }

    fn add_cloud(&mut self) {
        self.clouds.push(Cloud::new(self.dimensions_width));
    }
}
