#[derive(Default)]
#[allow(unused)]
pub struct Collectible {
    name: String,
}

impl Collectible {
    pub fn new() -> Self {
        Self {
            name: "Collect me pleaseee...".to_string(),
        }
    }
}
