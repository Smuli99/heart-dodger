use crate::point::Point2d;

// Enum for players direction
#[derive(Copy, Clone, Default)]
pub enum Direction {
    North,
    NorthEast,
    #[default]
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn as_coordinates(&self) -> Point2d<f64> {
        match self {
            Direction::North     => Point2d::new(0.0, -1.0),
            Direction::NorthEast => Point2d::new(0.7061, -0.7061),
            Direction::East      => Point2d::new(1.0, 0.0),
            Direction::SouthEast => Point2d::new(0.7061, 0.7061),
            Direction::South     => Point2d::new(0.0, 1.0),
            Direction::SouthWest => Point2d::new(-0.7061, 0.7061),
            Direction::West      => Point2d::new(-1.0, 0.0),
            Direction::NorthWest => Point2d::new(-0.7061, -0.7061),
        }
    }

    pub fn from_coordinates(x: f64, y: f64) -> Self {
        match (x, y) {
            (0.0, -1.0)        => Direction::North,
            (0.7061, -0.7061)  => Direction::NorthEast,
            (1.0, 0.0)         => Direction::East,
            (0.7061, 0.7061)   => Direction::SouthEast,
            (0.0, 1.0)         => Direction::South,
            (-0.7061, 0.7061)  => Direction::SouthWest,
            (-1.0, 0.0)        => Direction::West,
            (-0.7061, -0.7061) => Direction::NorthWest,
            _                  => Direction::default(),
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::North     => Direction::NorthWest,
            Direction::NorthEast => Direction::North,
            Direction::East      => Direction::NorthEast,
            Direction::SouthEast => Direction::East,
            Direction::South     => Direction::SouthEast,
            Direction::SouthWest => Direction::South,
            Direction::West      => Direction::SouthWest,
            Direction::NorthWest => Direction::West,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North     => Direction::NorthEast,
            Direction::NorthEast => Direction::East,
            Direction::East      => Direction::SouthEast,
            Direction::SouthEast => Direction::South,
            Direction::South     => Direction::SouthWest,
            Direction::SouthWest => Direction::West,
            Direction::West      => Direction::NorthWest,
            Direction::NorthWest => Direction::North,
        }
    }
}
