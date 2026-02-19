use crate::point::Point2d;
use crate::traits::Position;

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
}

// implementing Position trait
crate::position_impl!(Enemy, f64);
