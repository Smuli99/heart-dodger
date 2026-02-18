#[derive(Debug, Default)]
pub struct Wall {
    pub name: String,
}

impl Wall {
    // This will change
    pub fn new() -> Self {
        Self {
            name: "I'm a wall!".to_string(),
        }
    }
}
