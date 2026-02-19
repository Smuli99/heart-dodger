use crate::{
    point::Point2d,
    traits::Position,
    ui::draw::Draw, unit::Player,
};

use num::traits::ToPrimitive;
use std::fmt;

#[derive(Default)]
#[allow(unused)]
pub struct Enemy {
    speed: f64,
    position: Point2d<f64>,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
            position: Point2d::new(0.0, 0.0),
        }
    }

    pub fn with_speed(speed: f64) -> Self {
        Self {
            speed: speed,
            position: Point2d::new(0.0, 0.0),
        }
    }

    pub fn move_towards_player(&mut self, player: &Player) {
        let direction = (player.position() - self.position()).normalize();
        self.position += direction * self.speed;
    } 
}

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "👾")
    }
}

// implementing Position and Draw trait
crate::position_impl!(Enemy, f64);
crate::draw_impl!(Enemy, f64);
