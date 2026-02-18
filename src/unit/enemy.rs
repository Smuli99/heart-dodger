#[derive(Default)]
#[allow(unused)]
pub struct Enemy {
    speed: f64,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            speed: 1.0,
        }
    }

    pub fn with_speed(speed: f64) -> Self {
        Self {
            speed: speed,
        }
    }
}
