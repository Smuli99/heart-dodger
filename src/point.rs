use num::traits::NumAssign;
use crate::traits::Position;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point2d<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2d<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Default> Default for Point2d<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T> Position<T> for Point2d<T>
where 
    T: NumAssign + Copy,
{
    fn position(&self) -> Self {
        *self
    }
    
    fn set_position(&mut self, position: Self) {
        *self = position;
    }
}
