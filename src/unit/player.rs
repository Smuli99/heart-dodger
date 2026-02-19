use crate::{
    point::Point2d,
    traits::Position,
    ui::draw::Draw,
};

use num::traits::ToPrimitive;
use std::fmt;

#[derive(Default)]
pub struct Player {
    speed: f64,
    health: u8,
    position: Point2d<f64>,
}

impl Player {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> PlayerBuilder {
        PlayerBuilder::new()
    }

    pub fn is_alive(&self) -> bool {
        self.health != 0
    }

    pub fn take_damage(&mut self, damage: u8) {
        if damage >= self.health {
            self.health = 0;
        } else {
            self.health -= damage;
        }
    }

    pub fn health(&self) -> u8 {
        self.health
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "🚶🏽‍♂️‍➡️")
    }
}

//implement Position and Draw trait
crate::position_impl!(Player, f64);
crate::draw_impl!(Player, f64);

// Player Builder
pub struct PlayerBuilder {
    speed: f64,
    health: u8,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
            health: 10,
        }
    }

    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    pub fn health(mut self, health: u8) -> Self {
        self.health = health;
        self
    }

    pub fn build(self) -> Player {
        Player {
            speed: self.speed,
            health: self.health,
            position: Point2d::new(0.0, 0.0),
        }
    }
}
