use crate::unit::Player;
use std::io::Write;

pub struct Hud<'a> {
    score: u32,
    player: &'a Player,
    y_position: u16,
}

impl<'a> Hud<'a> {
    pub fn new(score: u32, player: &'a Player, y_position: u16) -> Self {
        Self {
            score: score,
            player: player,
            y_position: y_position,
        }
    }

    pub fn draw(&self, buffer: &mut impl Write) {
        use crossterm::{queue, cursor::MoveTo, style::Print};

        let text = format!(
            "Score: {} | Health: {} | Speed {:.1}",
            self.score,
            self.player.health(),
            self.player.speed()
        );

        queue!(
            buffer,
            MoveTo(0, self.y_position),
            Print(text)
        )
        .unwrap();
    }
}
