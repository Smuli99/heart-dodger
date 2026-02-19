use crate::{
    point::Point2d,
    traits::Position,
    ui::draw::Draw,
};

use num::traits::ToPrimitive;
use std::fmt;

#[derive(Default)]
#[allow(unused)]
pub struct Collectible {
    position: Point2d<u16>,
}

impl fmt::Display for Collectible {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "💵")
    }
}

// implementing Position and Draw trait
crate::position_impl!(Collectible, u16);
crate::draw_impl!(Collectible, u16);
