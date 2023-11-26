use std::ops::{BitOr, Index};

use bitflags::bitflags;

use crate::square::{File, Level, Rank, Square, NUM_RANKS};

const LEVEL_SHIFT: usize = 60;

bitflags! {
    #[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
    pub struct BitBoard: u64 {
        const EMPTY = 0;
        const Z0 = 1;
        const Z1 = 1 << 1;
        const Z2 = 1 << 2;
        const Z3 = 1 << 3;
        const Z4 = 1 << 4;
        const Z5 = 1 << 5;
        const Z6 = 1 << 6;
        const Z7 = 1 << 7;
        const Z8 = 1 << 8;
        const Z9 = 1 << 9;
        const A0 = 1 << 10;
        const A1 = 1 << 11;
        const A2 = 1 << 12;
        const A3 = 1 << 13;
        const A4 = 1 << 14;
        const A5 = 1 << 15;
        const A6 = 1 << 16;
        const A7 = 1 << 17;
        const A8 = 1 << 18;
        const A9 = 1 << 19;
        const B0 = 1 << 20;
        const B1 = 1 << 21;
        const B2 = 1 << 22;
        const B3 = 1 << 23;
        const B4 = 1 << 24;
        const B5 = 1 << 25;
        const B6 = 1 << 26;
        const B7 = 1 << 27;
        const B8 = 1 << 28;
        const B9 = 1 << 29;
        const C0 = 1 << 30;
        const C1 = 1 << 31;
        const C2 = 1 << 32;
        const C3 = 1 << 33;
        const C4 = 1 << 34;
        const C5 = 1 << 35;
        const C6 = 1 << 36;
        const C7 = 1 << 37;
        const C8 = 1 << 38;
        const C9 = 1 << 39;
        const D0 = 1 << 40;
        const D1 = 1 << 41;
        const D2 = 1 << 42;
        const D3 = 1 << 43;
        const D4 = 1 << 44;
        const D5 = 1 << 45;
        const D6 = 1 << 46;
        const D7 = 1 << 47;
        const D8 = 1 << 48;
        const D9 = 1 << 49;
        const E0 = 1 << 50;
        const E1 = 1 << 51;
        const E2 = 1 << 52;
        const E3 = 1 << 53;
        const E4 = 1 << 54;
        const E5 = 1 << 55;
        const E6 = 1 << 56;
        const E7 = 1 << 57;
        const E8 = 1 << 58;
        const E9 = 1 << 59;

        const WHITE_SET =   Self::A1.bits() | Self::A2.bits() | Self::A3.bits() | Self::A4.bits() |
                            Self::B1.bits() | Self::B2.bits() | Self::B3.bits() | Self::B4.bits() |
                            Self::C1.bits() | Self::C2.bits() | Self::C3.bits() | Self::C4.bits() |
                            Self::D1.bits() | Self::D2.bits() | Self::D3.bits() | Self::D4.bits();

        const NEUTRAL_SET = Self::A3.bits() | Self::A4.bits() | Self::A5.bits() | Self::A6.bits() |
                            Self::B3.bits() | Self::B4.bits() | Self::B5.bits() | Self::B6.bits() |
                            Self::C3.bits() | Self::C4.bits() | Self::C5.bits() | Self::C6.bits() |
                            Self::D3.bits() | Self::D4.bits() | Self::D5.bits() | Self::D6.bits();

        const BLACK_SET =   Self::A5.bits() | Self::A6.bits() | Self::A7.bits() | Self::A8.bits() |
                            Self::B5.bits() | Self::B6.bits() | Self::B7.bits() | Self::B8.bits() |
                            Self::C5.bits() | Self::C6.bits() | Self::C7.bits() | Self::C8.bits() |
                            Self::D5.bits() | Self::D6.bits() | Self::D7.bits() | Self::D8.bits();

        const QL1_SET =     Self::Z0.bits() | Self::Z1.bits() |
                            Self::A0.bits() | Self::A1.bits();

        const QL2_SET =     Self::Z4.bits() | Self::Z5.bits() |
                            Self::A4.bits() | Self::A5.bits();

        const QL3_SET =     Self::Z2.bits() | Self::Z3.bits() |
                            Self::A2.bits() | Self::A3.bits();

        const QL4_SET =     Self::Z6.bits() | Self::Z7.bits() |
                            Self::A6.bits() | Self::A7.bits();

        const QL5_SET =     Self::Z4.bits() | Self::Z5.bits() |
                            Self::A4.bits() | Self::A5.bits();

        const QL6_SET =     Self::Z8.bits() | Self::Z9.bits() |
                            Self::A8.bits() | Self::A9.bits();

        const KL1_SET =     Self::D0.bits() | Self::D1.bits() |
                            Self::E0.bits() | Self::E1.bits();

        const KL2_SET =     Self::D4.bits() | Self::D5.bits() |
                            Self::E4.bits() | Self::E5.bits();

        const KL3_SET =     Self::D2.bits() | Self::D3.bits() |
                            Self::E2.bits() | Self::E3.bits();

        const KL4_SET =     Self::D6.bits() | Self::D7.bits() |
                            Self::E6.bits() | Self::E7.bits();

        const KL5_SET =     Self::D4.bits() | Self::D5.bits() |
                            Self::E4.bits() | Self::E5.bits();

        const KL6_SET =     Self::D8.bits() | Self::D9.bits() |
                            Self::E8.bits() | Self::E9.bits();
    }
}

impl BitBoard {
    pub const WHITE: BitBoard = BitBoard::from_bits_retain(0b0001 << LEVEL_SHIFT);
    pub const NEUTRAL: BitBoard = BitBoard::from_bits_retain(0b0010 << LEVEL_SHIFT);
    pub const BLACK: BitBoard = BitBoard::from_bits_retain(0b0011 << LEVEL_SHIFT);
    pub const QL1: BitBoard = BitBoard::from_bits_retain(0b0100 << LEVEL_SHIFT);
    pub const QL2: BitBoard = BitBoard::from_bits_retain(0b0101 << LEVEL_SHIFT);
    pub const QL3: BitBoard = BitBoard::from_bits_retain(0b0110 << LEVEL_SHIFT);
    pub const QL4: BitBoard = BitBoard::from_bits_retain(0b0111 << LEVEL_SHIFT);
    pub const QL5: BitBoard = BitBoard::from_bits_retain(0b1000 << LEVEL_SHIFT);
    pub const QL6: BitBoard = BitBoard::from_bits_retain(0b1001 << LEVEL_SHIFT);
    pub const KL1: BitBoard = BitBoard::from_bits_retain(0b1010 << LEVEL_SHIFT);
    pub const KL2: BitBoard = BitBoard::from_bits_retain(0b1011 << LEVEL_SHIFT);
    pub const KL3: BitBoard = BitBoard::from_bits_retain(0b1100 << LEVEL_SHIFT);
    pub const KL4: BitBoard = BitBoard::from_bits_retain(0b1101 << LEVEL_SHIFT);
    pub const KL5: BitBoard = BitBoard::from_bits_retain(0b1110 << LEVEL_SHIFT);
    pub const KL6: BitBoard = BitBoard::from_bits_retain(0b1111 << LEVEL_SHIFT);

    const LEVEL_MASK: BitBoard = BitBoard::from_bits_retain(0b1111 << LEVEL_SHIFT);

    pub fn from_square(square: &Square) -> Self {
        let shift = square.rank as u8 + square.file as u8 * NUM_RANKS;

        Self::from_bits_retain((1 << shift) | ((square.level as u64) << LEVEL_SHIFT))
    }

    pub fn into_square(self) -> Square {
        let bits = self.bits();
        let shift = bits.trailing_zeros() as u8;

        Square::new(
            Rank::from_u8(shift % NUM_RANKS),
            File::from_u8(shift / NUM_RANKS),
            Level::from_u8((bits >> LEVEL_SHIFT) as u8),
        )
    }

    pub fn remove_level(self) -> Self {
        self & !Self::LEVEL_MASK
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum BoardType {
    White,
    Neutral,
    Black,
    WhiteQueen,
    WhiteKing,
    BlackQueen,
    BlackKing,
}

impl BoardType {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::White,
            Self::Neutral,
            Self::Black,
            Self::WhiteQueen,
            Self::WhiteKing,
            Self::BlackQueen,
            Self::BlackKing,
        ]
        .iter()
        .copied()
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct BitBoardSet {
    raw: [BitBoard; 7],
}

impl BitBoardSet {
    pub fn new() -> Self {
        Self {
            raw: [BitBoard::EMPTY; 7],
        }
    }

    pub fn union(&self) -> BitBoard {
        self.raw.iter().fold(BitBoard::EMPTY, |acc, x| acc | *x)
    }
}

impl Index<BoardType> for BitBoardSet {
    type Output = BitBoard;

    fn index(&self, index: BoardType) -> &Self::Output {
        &self.raw[index as usize]
    }
}

impl BitOr<Self> for BitBoardSet {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union() | rhs.union()
    }
}

impl BitOr<&Self> for BitBoardSet {
    type Output = BitBoard;

    fn bitor(self, rhs: &Self) -> Self::Output {
        self.union() | rhs.union()
    }
}
