use std::ops::{Index, BitOr};

use crate::{square::Color, bit_board::{BitBoardSet, BitBoard}};

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
        self.raw.iter().fold(BitBoard::EMPTY, |acc, x| acc | x.union())
    }
}

impl Index<Color> for ColorMask {
    type Output = BitBoardSet;

    fn index(&self, index: Color) -> &Self::Output {
        &self.raw[index as usize]
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
