#[derive(Default)]
#[allow(unused)]
pub struct Enemy {
    name: String,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            name: "Thanos".to_string(),
        }
    }
}
