use num::traits::NumAssign;
use rand::{
    distr::{uniform::SampleUniform, StandardUniform},
    prelude::Distribution,
    rngs::ThreadRng,
    RngExt,
};
use std::ops::Range;

use crate::point::Point2d;

// Generic Position trait
pub trait Position<T: NumAssign + Copy> {
    fn position(&self) -> Point2d<T>;
    fn set_position(&mut self, position: Point2d<T>);
    fn set_rand_position(&mut self, rng: &mut ThreadRng, x_range: Range<T>, y_range: Range<T>)
    where
        T: PartialOrd + SampleUniform,
        StandardUniform: Distribution<T>,
    {
        let new_position = Point2d::new(rng.random_range(x_range), rng.random_range(y_range));
        self.set_position(new_position);
    }
}

// macro for impl
#[macro_export]
macro_rules! position_impl {
    ($struct_name: ident, $ty: ty) => {
        impl Position<$ty> for $struct_name {
            fn position(&self) -> Point2d<$ty> {
                self.position
            }

            fn set_position(&mut self, position: Point2d<$ty>) {
                self.position = position;
            }
        }
    }
}
