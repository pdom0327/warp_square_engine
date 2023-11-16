use crate::{
    square::{BitBoard, Color}, color_mask::ColorMask,
};

// const DEFAULT_SET: [(BitBoard, (Piece, Color)); 32] = [
//     (BitBoard::QW0, (Piece::Rook, Color::White)),
//     (BitBoard::QW1, (Piece::Queen, Color::White)),
//     (BitBoard::QW2, (Piece::Pawn, Color::White)),
//     (BitBoard::QW3, (Piece::Pawn, Color::White)),
//     (BitBoard::W0, (Piece::Knight, Color::White)),
//     (BitBoard::W1, (Piece::Bishop, Color::White)),
//     (BitBoard::W2, (Piece::Bishop, Color::White)),
//     (BitBoard::W3, (Piece::Knight, Color::White)),
//     (BitBoard::W4, (Piece::Pawn, Color::White)),
//     (BitBoard::W5, (Piece::Pawn, Color::White)),
//     (BitBoard::W6, (Piece::Pawn, Color::White)),
//     (BitBoard::W7, (Piece::Pawn, Color::White)),
//     (BitBoard::KW0, (Piece::King, Color::White)),
//     (BitBoard::KW1, (Piece::Rook, Color::White)),
//     (BitBoard::KW2, (Piece::Pawn, Color::White)),
//     (BitBoard::KW3, (Piece::Pawn, Color::White)),
//     (BitBoard::QB0, (Piece::Pawn, Color::Black)),
//     (BitBoard::QB1, (Piece::Pawn, Color::Black)),
//     (BitBoard::QB2, (Piece::Rook, Color::Black)),
//     (BitBoard::QB3, (Piece::Queen, Color::Black)),
//     (BitBoard::B8, (Piece::Pawn, Color::Black)),
//     (BitBoard::B9, (Piece::Pawn, Color::Black)),
//     (BitBoard::BA, (Piece::Pawn, Color::Black)),
//     (BitBoard::BB, (Piece::Pawn, Color::Black)),
//     (BitBoard::BC, (Piece::Knight, Color::Black)),
//     (BitBoard::BD, (Piece::Bishop, Color::Black)),
//     (BitBoard::BE, (Piece::Bishop, Color::Black)),
//     (BitBoard::BF, (Piece::King, Color::Black)),
//     (BitBoard::KB0, (Piece::Pawn, Color::Black)),
//     (BitBoard::KB1, (Piece::Pawn, Color::Black)),
//     (BitBoard::KB2, (Piece::King, Color::Black)),
//     (BitBoard::KB3, (Piece::Rook, Color::Black)),
// ];

pub struct BoardSet {
    turn: Color,
    occupied_mask: ColorMask,
}

impl BoardSet {
    fn new() -> Self {
        Self {
            turn: Color::White,
            occupied_mask: ColorMask(BitBoard::Empty, BitBoard::Empty)
        }
    }

    fn push() -> bool {
        true
    }
}
