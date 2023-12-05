use crate::{
    bit_board::BitBoard,
    color_mask::ColorMask,
    square::{Color, Square},
};

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const NUM_PIECES: usize = 6;

impl PieceType {
    pub fn get_char(&self, color: Color) -> &'static str {
        match color {
            Color::White => match self {
                Self::Pawn => "P",
                Self::Knight => "N",
                Self::Bishop => "B",
                Self::Rook => "R",
                Self::Queen => "Q",
                Self::King => "K",
            },
            Color::Black => match self {
                Self::Pawn => "p",
                Self::Knight => "n",
                Self::Bishop => "b",
                Self::Rook => "r",
                Self::Queen => "q",
                Self::King => "k",
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub position: BitBoard,
    pub attacks: BitBoard,
}

impl Piece {
    pub fn new(position: BitBoard, piece_type: PieceType, color: Color) -> Self {
        Self {
            piece_type,
            color,
            position,
            attacks: BitBoard::EMPTY,
        }
    }

    pub fn get_square(&self) -> Square {
        BitBoard::into_square(&self.position)
    }

    pub fn get_char(&self) -> &'static str {
        self.piece_type.get_char(self.color)
    }

    pub fn update_attacks(&mut self, occupied: &ColorMask) {
        self.attacks = match self.piece_type {
            PieceType::Pawn => self.compute_pawn_attacks(occupied),
            PieceType::Knight => self.compute_knight_attacks(occupied),
            PieceType::Bishop => self.compute_bishop_attacks(occupied),
            PieceType::Rook => self.compute_rook_attacks(occupied),
            PieceType::Queen => self.compute_queen_attacks(occupied),
            PieceType::King => self.compute_king_attacks(occupied),
        };
    }

    pub fn compute_pawn_attacks(&self, occupied: &ColorMask) -> BitBoard {
        todo!()
    }

    pub fn compute_knight_attacks(&self, occupied: &ColorMask) -> BitBoard {
        todo!()
    }

    pub fn compute_bishop_attacks(&self, occupied: &ColorMask) -> BitBoard {
        todo!()
    }

    pub fn compute_rook_attacks(&self, occupied: &ColorMask) -> BitBoard {
        todo!()
    }

    pub fn compute_queen_attacks(&self, occupied: &ColorMask) -> BitBoard {
        todo!()
    }

    pub fn compute_king_attacks(&self, occupied: &ColorMask) -> BitBoard {
        todo!()
    }
}
