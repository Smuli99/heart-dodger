use num::traits::NumAssign;
use crate::traits::Position;

use std::ops::{Add, AddAssign, Sub, Mul};

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

// for Point2d<f64>
impl Point2d<f64> {
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    pub fn to_u16(&self) -> Point2d<u16> {
        Point2d {
            x: self.x as u16,
            y: self.y as u16,
        }
    }

    pub fn normalize(&self) -> Self {
        let magnitude = (self.x * self.x + self.y * self.y).sqrt();

        // if dividing with zero
        if magnitude == 0.0 {
            return Self { x: 0.0, y: 0.0 };
        }

        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }
}

// Add, AddAssign, Sub, Mul impl
impl<T> Add for Point2d<T> 
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Point2d<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Point2d<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Point2d<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// Position impl
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
