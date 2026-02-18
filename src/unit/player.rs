#[derive(Debug, Default)]
pub struct Player {
    pub name: String,
}

impl Player {
    // this will change
    pub fn new() -> Self {
        Self {
            name: "Shlmatan".to_string(),
        }
    }
}
