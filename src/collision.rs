use crate::config;
use crate::obstacle::Obstacle;
use crate::trex::Trex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CollisionBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl CollisionBox {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.height + self.y > other.y
    }

    pub fn adjusted_by(&self, adjustment: &Self) -> Self {
        Self::new(
            self.x + adjustment.x,
            self.y + adjustment.y,
            self.width,
            self.height,
        )
    }
}

pub fn check_for_collision(
    obstacle: &Obstacle,
    trex: &Trex,
) -> Option<(CollisionBox, CollisionBox)> {
    let trex_box = CollisionBox::new(
        trex.x_pos + 1.0,
        trex.y_pos + 1.0,
        config::TREX_WIDTH - 2.0,
        config::TREX_HEIGHT - 2.0,
    );
    let obstacle_box = CollisionBox::new(
        obstacle.x_pos + 1.0,
        obstacle.y_pos + 1.0,
        obstacle.type_config.width * obstacle.size as f64 - 2.0,
        obstacle.type_config.height - 2.0,
    );

    if trex_box.intersects(&obstacle_box) {
        let trex_boxes = trex.collision_boxes();
        for trex_collision_box in trex_boxes {
            for obstacle_collision_box in &obstacle.collision_boxes {
                let adjusted_trex_box = trex_collision_box.adjusted_by(&trex_box);
                let adjusted_obstacle_box = obstacle_collision_box.adjusted_by(&obstacle_box);
                if adjusted_trex_box.intersects(&adjusted_obstacle_box) {
                    return Some((adjusted_trex_box, adjusted_obstacle_box));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_aabb_intersection() {
        let a = CollisionBox::new(0.0, 0.0, 10.0, 10.0);
        let b = CollisionBox::new(9.0, 9.0, 10.0, 10.0);
        let c = CollisionBox::new(10.0, 10.0, 1.0, 1.0);
        assert!(a.intersects(&b));
        assert!(!a.intersects(&c));
    }
}
