use crate::point::Point2d;
use crate::traits::Position;

#[derive(Debug, Default)]
pub struct Wall {
    position: Point2d<u16>,
}

impl Wall {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            position: Point2d::new(x, y),
        }
    }
}

// implement Position trait
crate::position_impl!(Wall, u16);
