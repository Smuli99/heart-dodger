use crate::{
    point::Point2d,
    traits::Position,
    ui::draw::Draw,
};

use num::traits::ToPrimitive;
use std::fmt;

#[derive(Default)]
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

impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "|")
    }
}

// implement Position and Draw trait
crate::position_impl!(Wall, u16);
crate::draw_impl!(Wall, u16);
