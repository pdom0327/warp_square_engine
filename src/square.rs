use std::mem::transmute;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum Rank {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Rank {
    pub fn from_u8(i: i8) -> Self {
        unsafe { transmute(i.clamp(Rank::Zero as i8, Rank::Nine as i8)) }
    }

    pub fn down(&self) -> Self {
        Self::from_u8(*self as i8 - 1)
    }

    pub fn up(&self) -> Self {
        Self::from_u8(*self as i8 + 1)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum File {
    Z,
    A,
    B,
    C,
    D,
    E,
}

impl File {
    pub fn from_u8(i: i8) -> Self {
        unsafe { transmute(i.clamp(File::Z as i8, File::E as i8)) }
    }

    pub fn left(&self) -> Self {
        Self::from_u8(*self as i8 - 1)
    }

    pub fn right(&self) -> Self {
        Self::from_u8(*self as i8 + 1)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum Level {
    White,
    Neutral,
    Black,
    QL1,
    QL2,
    QL3,
    QL4,
    QL5,
    QL6,
    KL1,
    KL2,
    KL3,
    KL4,
    KL5,
    KL6,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct Square {
    pub rank: Rank,
    pub file: File,
    pub level: Level,
}

impl Square {
    pub fn new(rank: Rank, file: File, level: Level) -> Square {
        Square { rank, file, level }
    }

    pub fn down(&self) -> Self {
        Self::new(self.rank.down(), self.file, self.level)
    }

    pub fn up(&self) -> Self {
        Self::new(self.rank.up(), self.file, self.level)
    }

    pub fn left(&self) -> Self {
        Self::new(self.rank, self.file.left(), self.level)
    }

    pub fn right(&self) -> Self {
        Self::new(self.rank, self.file.right(), self.level)
    }

    pub fn forward(&self, color: Color) -> Self {
        match color {
            Color::White => self.up(),
            Color::Black => self.down(),
        }
    }

    pub fn backward(&self, color: Color) -> Self {
        match color {
            Color::White => self.down(),
            Color::Black => self.up(),
        }
    }
}
