use std::ops::{BitOr, Index, IndexMut};

use crate::{
    bit_board::{BitBoard, BitBoardSet},
    square::Color,
};

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct ColorMask {
    raw: [BitBoardSet; 2],
}

impl ColorMask {
    pub fn new() -> Self {
        Self {
            raw: [BitBoardSet::new(), BitBoardSet::new()],
        }
    }

    pub fn union(&self) -> BitBoard {
        self.raw
            .iter()
            .fold(BitBoard::EMPTY, |acc, x| acc | x.union())
    }
}

impl Index<Color> for ColorMask {
    type Output = BitBoardSet;

    fn index(&self, index: Color) -> &Self::Output {
        &self.raw[index as usize]
    }
}

impl IndexMut<Color> for ColorMask {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self.raw[index as usize]
    }
}

impl BitOr<Self> for ColorMask {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union() | rhs.union()
    }
}

impl BitOr<&Self> for ColorMask {
    type Output = BitBoard;

    fn bitor(self, rhs: &Self) -> Self::Output {
        self.union() | rhs.union()
    }
}
