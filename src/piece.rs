use crate::square::Color;

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub const NUM_PIECES: usize = 6;

pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }
}
