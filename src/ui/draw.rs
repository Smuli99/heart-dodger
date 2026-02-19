use crate::{
    traits::Position,
    // unit::{Collectible, Enemy, Player, Wall},
};

use std::{fmt::Display, io::Write};

// use crossterm::style::Stylize;
use num::{traits::NumAssign, NumCast};

pub trait Draw<T: NumAssign + Copy + NumCast>: Position<T> + Display {
    fn draw(&self, buffer: &mut impl Write) {
        let position = self.position();

        crossterm::queue!(
            buffer,
            crossterm::cursor::MoveTo(
                position
                    .x
                    .to_f64()
                    .expect("could not convert position x to f64")
                    .round() as u16 + 1,
                position
                    .y
                    .to_f64()
                    .expect("could not convert position y to f64")
                    .round() as u16 + 1,
            ),
            crossterm::style::Print(self)
        )
        .unwrap();
    }
}

// macro impl
#[macro_export]
macro_rules! draw_impl {
    ($struct_name: ident, $ty: ty) => {
        impl Draw<$ty> for $struct_name
        where
            $ty: num::traits::NumAssign + Copy + num::NumCast,
        {
            fn draw(&self, buffer: &mut impl std::io::Write) {
                let position = self.position();

                crossterm::queue!(
                    buffer,
                    crossterm::cursor::MoveTo(
                        position.x.to_f64().expect("could not convert position x to f64").round() as u16 + 1,
                        position.y.to_f64().expect("could not convert position y to f64").round() as u16 + 1,
                    ),
                    crossterm::style::Print(self)
                )
                .unwrap();
            }
        }
    };
}
