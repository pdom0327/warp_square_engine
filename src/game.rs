use std::collections::{HashMap, HashSet};

use crate::{
    piece::Piece,
    // color_mask::ColorMask,
    piece_move::PieceMove,
    square::{Color, File, Level, Rank, Square},
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

pub struct GameSnapshot {
    pub turn: Color,
    // occupied_mask: ColorMask,
    pub pieces: HashMap<Square, Piece>,
}

impl GameSnapshot {
    fn new(turn: Color, pieces: HashMap<Square, Piece>) -> Self {
        Self { turn, pieces }
    }

    fn restore(&self, board_set: &mut Game) {
        board_set.turn = self.turn;
        board_set.pieces = self.pieces.clone();
    }
}

pub struct Game {
    move_stack: Vec<(PieceMove, GameSnapshot)>,
    turn: Color,
    // occupied_mask: ColorMask,
    pieces: HashMap<Square, Piece>,
}

impl Game {
    const WHITE_SET: [(Rank, File); 16] = [
        (Rank::One, File::A),
        (Rank::Two, File::A),
        (Rank::Three, File::A),
        (Rank::Four, File::A),
        (Rank::One, File::B),
        (Rank::Two, File::B),
        (Rank::Three, File::B),
        (Rank::Four, File::B),
        (Rank::One, File::C),
        (Rank::Two, File::C),
        (Rank::Three, File::C),
        (Rank::Four, File::C),
        (Rank::One, File::D),
        (Rank::Two, File::D),
        (Rank::Three, File::D),
        (Rank::Four, File::D),
    ];

    const NEUTRAL_SET: [(Rank, File); 16] = [
        (Rank::Three, File::A),
        (Rank::Four, File::A),
        (Rank::Five, File::A),
        (Rank::Six, File::A),
        (Rank::Three, File::B),
        (Rank::Four, File::B),
        (Rank::Five, File::B),
        (Rank::Six, File::B),
        (Rank::Three, File::C),
        (Rank::Four, File::C),
        (Rank::Five, File::C),
        (Rank::Six, File::C),
        (Rank::Three, File::D),
        (Rank::Four, File::D),
        (Rank::Five, File::D),
        (Rank::Six, File::D),
    ];

    const BLACK_SET: [(Rank, File); 16] = [
        (Rank::Five, File::A),
        (Rank::Six, File::A),
        (Rank::Seven, File::A),
        (Rank::Eight, File::A),
        (Rank::Five, File::B),
        (Rank::Six, File::B),
        (Rank::Seven, File::B),
        (Rank::Eight, File::B),
        (Rank::Five, File::C),
        (Rank::Six, File::C),
        (Rank::Seven, File::C),
        (Rank::Eight, File::C),
        (Rank::Five, File::D),
        (Rank::Six, File::D),
        (Rank::Seven, File::D),
        (Rank::Eight, File::D),
    ];

    pub fn new() -> Self {
        Self {
            move_stack: Vec::new(),
            turn: Color::White,
            // occupied_mask: ColorMask(BitBoard::Empty, BitBoard::Empty),
            pieces: HashMap::new(),
        }
    }

    fn add_piece(&mut self, square: &Square, piece: Piece) -> Option<Piece> {
        self.pieces.insert(square.clone(), piece)
    }

    fn remove_piece(&mut self, square: &Square) -> Option<Piece> {
        self.pieces.remove(square)
    }

    fn pass_turn(&mut self) {
        if self.turn == Color::White {
            self.turn = Color::Black;
        } else {
            self.turn = Color::White;
        }
    }

    pub fn legal() {}

    pub fn push_move(&mut self, piece_move: PieceMove) -> Result<Option<Piece>, ()> {
        let snapshot = GameSnapshot::new(self.turn, self.pieces.clone());

        let captured_piece = match self.pieces.get(&piece_move.source) {
            Some(piece) => {
                self.remove_piece(&piece_move.source);
                self.add_piece(&piece_move.destination, *piece)
            }
            None => return Err(()),
        };

        self.move_stack.push((piece_move, snapshot));
        self.pass_turn();

        Ok(captured_piece)
    }

    pub fn pop_move(&mut self) -> Result<PieceMove, ()> {
        self.pass_turn();
        match self.move_stack.pop() {
            Some((piece_move, snapshot)) => {
                snapshot.restore(self);
                Ok(piece_move)
            }
            None => Err(()),
        }
    }

    pub fn validate_square(&self, square: &Square) -> bool {
        match square.level {
            Level::White => Self::WHITE_SET.contains(&(square.rank, square.file)),
            Level::Neutral => Self::NEUTRAL_SET.contains(&(square.rank, square.file)),
            Level::Black => Self::BLACK_SET.contains(&(square.rank, square.file)),
            _ => false,
        }
    }
}
