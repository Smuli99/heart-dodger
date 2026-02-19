use crate::point::Point2d;
use crate::traits::Position;

#[derive(Default)]
#[allow(unused)]
pub struct Collectible {
    position: Point2d<u16>,
}

// implementing Position trait
crate::position_impl!(Collectible, u16);
