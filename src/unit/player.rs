use crate::{
    point::Point2d,
    traits::Position,
    ui::draw::Draw,
    enums::Direction,
};

use num::traits::ToPrimitive;
use std::fmt;

#[derive(Default)]
pub struct Player {
    speed: f64,
    health: u8,
    position: Point2d<f64>,
    direction: Direction,
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

    pub fn accelerate(&mut self) {
        // player max speed 1.0
        self.speed = (self.speed + 0.1).min(1.0);        
    }

    pub fn decelerate(&mut self) {
        // player min speed 0.0
        self.speed = (self.speed - 0.1).max(0.0);
    }

    pub fn move_forward(&mut self) {
        self.position += self.direction.as_coordinates() * self.speed;
    }

    pub fn forward_position(&self) -> Point2d<u16> {
        let next = self.position + self.direction.as_coordinates() * self.speed;

        Point2d::new(
            next.x.round() as u16,
            next.y.round() as u16,
        )
    }

    pub fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    pub fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }
}

// Display trait impl
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol: &str = match self.direction {
            Direction::North     => "🧍🏽‍♂️",
            Direction::NorthEast => "🚶🏽‍♂️‍➡️",
            Direction::East      => "🏃🏽‍♂️‍➡️",
            Direction::SouthEast => "🚶🏽‍♂️‍➡️",
            Direction::South     => "🧍🏽‍♂️",
            Direction::SouthWest => "🚶🏽‍♂️",
            Direction::West      => "🏃🏽‍♂️",
            Direction::NorthWest => "🚶🏽‍♂️",
        };
        
        write!(f, "{}", symbol)
    }
}

//implement Position and Draw trait
crate::position_impl!(Player, f64);
crate::draw_impl!(Player, f64);

// Player Builder
pub struct PlayerBuilder {
    speed: f64,
    health: u8,
    position: Point2d<f64>,
    direction: Direction,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
            health: 10,
            position: Point2d::new(1.0, 1.0),
            direction: Direction::default(),
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

    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Point2d::new(x, y);
        self
    }

    pub fn direction(mut self, x: f64, y: f64) -> Self {
        self.direction = Direction::from_coordinates(x, y);
        self
    }

    pub fn build(self) -> Player {
        Player {
            speed: self.speed,
            health: self.health,
            position: self.position,
            direction: self.direction,
        }
    }
}
