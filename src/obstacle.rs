use rand::Rng;

use crate::collision::CollisionBox;
use crate::config;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObstacleKind {
    CactusSmall,
    CactusLarge,
    Pterodactyl,
}

#[derive(Clone, Copy, Debug)]
pub struct ObstacleType {
    pub kind: ObstacleKind,
    pub width: f64,
    pub height: f64,
    pub y_pos: &'static [f64],
    pub multiple_speed: f64,
    pub min_gap: f64,
    pub min_speed: f64,
    pub collision_boxes: &'static [CollisionBox],
    pub num_frames: usize,
    pub frame_rate: f64,
    pub speed_offset: f64,
}

pub const CACTUS_SMALL_BOXES: &[CollisionBox] = &[
    CollisionBox::new(0.0, 7.0, 5.0, 27.0),
    CollisionBox::new(4.0, 0.0, 6.0, 34.0),
    CollisionBox::new(10.0, 4.0, 7.0, 14.0),
];

pub const CACTUS_LARGE_BOXES: &[CollisionBox] = &[
    CollisionBox::new(0.0, 12.0, 7.0, 38.0),
    CollisionBox::new(8.0, 0.0, 7.0, 49.0),
    CollisionBox::new(13.0, 10.0, 10.0, 38.0),
];

pub const PTERODACTYL_BOXES: &[CollisionBox] = &[
    CollisionBox::new(15.0, 15.0, 16.0, 5.0),
    CollisionBox::new(18.0, 21.0, 24.0, 6.0),
    CollisionBox::new(2.0, 14.0, 4.0, 3.0),
    CollisionBox::new(6.0, 10.0, 4.0, 7.0),
    CollisionBox::new(10.0, 8.0, 6.0, 9.0),
];

const SMALL_Y: &[f64] = &[105.0];
const LARGE_Y: &[f64] = &[90.0];
const PTERODACTYL_Y: &[f64] = &[100.0, 75.0, 50.0];

pub const OBSTACLE_TYPES: &[ObstacleType] = &[
    ObstacleType {
        kind: ObstacleKind::CactusSmall,
        width: 17.0,
        height: 35.0,
        y_pos: SMALL_Y,
        multiple_speed: 4.0,
        min_gap: 120.0,
        min_speed: 0.0,
        collision_boxes: CACTUS_SMALL_BOXES,
        num_frames: 1,
        frame_rate: 0.0,
        speed_offset: 0.0,
    },
    ObstacleType {
        kind: ObstacleKind::CactusLarge,
        width: 25.0,
        height: 50.0,
        y_pos: LARGE_Y,
        multiple_speed: 7.0,
        min_gap: 120.0,
        min_speed: 0.0,
        collision_boxes: CACTUS_LARGE_BOXES,
        num_frames: 1,
        frame_rate: 0.0,
        speed_offset: 0.0,
    },
    ObstacleType {
        kind: ObstacleKind::Pterodactyl,
        width: 46.0,
        height: 40.0,
        y_pos: PTERODACTYL_Y,
        multiple_speed: 999.0,
        min_gap: 150.0,
        min_speed: 8.5,
        collision_boxes: PTERODACTYL_BOXES,
        num_frames: 2,
        frame_rate: 1000.0 / 6.0,
        speed_offset: 0.8,
    },
];

pub struct Obstacle {
    pub type_config: ObstacleType,
    pub size: u32,
    pub remove: bool,
    pub x_pos: f64,
    pub y_pos: f64,
    pub width: f64,
    pub collision_boxes: Vec<CollisionBox>,
    pub gap: f64,
    pub speed_offset: f64,
    pub current_frame: usize,
    pub following_obstacle_created: bool,
    timer: f64,
}

impl Obstacle {
    pub fn new(type_config: ObstacleType, gap_coefficient: f64, speed: f64, x_offset: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut obstacle = Self {
            type_config,
            size: rng.gen_range(1..=config::MAX_OBSTACLE_LENGTH),
            remove: false,
            x_pos: config::DEFAULT_WIDTH + x_offset,
            y_pos: 0.0,
            width: 0.0,
            collision_boxes: type_config.collision_boxes.to_vec(),
            gap: 0.0,
            speed_offset: 0.0,
            current_frame: 0,
            following_obstacle_created: false,
            timer: 0.0,
        };
        obstacle.init(gap_coefficient, speed);
        obstacle
    }

    pub fn update(&mut self, delta_ms: f64, mut speed: f64) {
        if self.remove {
            return;
        }

        if self.type_config.speed_offset != 0.0 {
            speed += self.speed_offset;
        }
        self.x_pos -= (speed * config::FPS / 1000.0 * delta_ms).floor();

        if self.type_config.num_frames > 1 {
            self.timer += delta_ms;
            if self.timer >= self.type_config.frame_rate {
                self.current_frame = if self.current_frame == self.type_config.num_frames - 1 {
                    0
                } else {
                    self.current_frame + 1
                };
                self.timer = 0.0;
            }
        }

        if !self.is_visible() {
            self.remove = true;
        }
    }

    pub fn is_visible(&self) -> bool {
        self.x_pos + self.width > 0.0
    }

    pub fn sprite_key(&self) -> &'static str {
        match self.type_config.kind {
            ObstacleKind::CactusSmall => match self.size {
                1 => "cactus_small_1",
                2 => "cactus_small_2",
                _ => "cactus_small_3",
            },
            ObstacleKind::CactusLarge => match self.size {
                1 => "cactus_large_1",
                2 => "cactus_large_2",
                _ => "cactus_large_3",
            },
            ObstacleKind::Pterodactyl => {
                if self.current_frame == 0 {
                    "pterodactyl_1"
                } else {
                    "pterodactyl_2"
                }
            }
        }
    }

    fn init(&mut self, gap_coefficient: f64, speed: f64) {
        if self.size > 1 && self.type_config.multiple_speed > speed {
            self.size = 1;
        }
        self.width = self.type_config.width * self.size as f64;
        self.y_pos =
            self.type_config.y_pos[rand::thread_rng().gen_range(0..self.type_config.y_pos.len())];

        if self.size > 1 && self.collision_boxes.len() >= 3 {
            let first_width = self.collision_boxes[0].width;
            let third_width = self.collision_boxes[2].width;
            self.collision_boxes[1].width = self.width - first_width - third_width;
            self.collision_boxes[2].x = self.width - third_width;
        }

        if self.type_config.speed_offset != 0.0 {
            self.speed_offset = if rand::random::<bool>() {
                self.type_config.speed_offset
            } else {
                -self.type_config.speed_offset
            };
        }

        self.gap = self.get_gap(gap_coefficient, speed);
    }

    fn get_gap(&self, gap_coefficient: f64, speed: f64) -> f64 {
        let min_gap = (self.width * speed + self.type_config.min_gap * gap_coefficient).round();
        let max_gap = (min_gap * 1.5).round();
        rand::thread_rng().gen_range(min_gap..=max_gap)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_size_is_gated_by_speed() {
        for _ in 0..20 {
            let obstacle = Obstacle::new(OBSTACLE_TYPES[1], config::GAP_COEFFICIENT, 6.0, 25.0);
            assert_eq!(obstacle.size, 1);
        }
    }

    #[test]
    fn gap_scales_with_speed() {
        let slow = Obstacle::new(OBSTACLE_TYPES[0], config::GAP_COEFFICIENT, 6.0, 17.0);
        let fast = Obstacle::new(OBSTACLE_TYPES[0], config::GAP_COEFFICIENT, 12.0, 17.0);
        assert!(fast.gap >= slow.type_config.min_gap * config::GAP_COEFFICIENT);
        assert!(slow.gap >= slow.width * 6.0 + slow.type_config.min_gap * config::GAP_COEFFICIENT);
    }
}
