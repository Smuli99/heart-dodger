use crate::unit::{Collectible, Enemy, Player, Wall};

#[allow(unused)]
pub struct Game {
    enemies: Vec<Enemy>,
    walls: Vec<Wall>,
    collectible: Collectible,
    player: Player,
}

impl Game {
    pub fn new() -> Self {
        Self {
            enemies: vec![Enemy::default(), Enemy::default()],
            walls: vec![Wall::default(), Wall::default(), Wall::default()],
            collectible: Collectible::default(),
            player: Player::default(),
        }
    }
}
