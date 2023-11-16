use std::ops::Index;

use crate::square::{Color, BitBoard};

pub struct ColorMask(pub BitBoard, pub BitBoard);

impl Index<Color> for ColorMask {
    type Output = BitBoard;

    fn index(&self, index: Color) -> &Self::Output {
        match index {
            Color::White => &self.0,
            Color::Black => &self.1,
        }
    }
}
