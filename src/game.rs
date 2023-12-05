use crate::{
    bit_board::{BitBoard, BoardType},
    piece::{Piece, PieceType},
    piece_move::PieceMove,
    square::{Color, Level, Rank}, color_mask::ColorMask,
};

pub struct BoardSnapshot {
    pieces: Vec<Piece>,
    captured_pieces: Vec<Piece>,
}

impl BoardSnapshot {
    pub fn new(board: &Board) -> Self {
        Self {
            pieces: board.pieces.clone(),
            captured_pieces: board.captured_pieces.clone(),
        }
    }

    pub fn restore(&self, board: &mut Board) {
        board.pieces = self.pieces.clone();
        board.captured_pieces = self.captured_pieces.clone();
    }
}

pub struct Board {
    pub pieces: Vec<Piece>,
    pub captured_pieces: Vec<Piece>,
    pub occupied: ColorMask,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: Vec::new(),
            captured_pieces: Vec::new(),
            occupied: ColorMask::new(),
        }
    }

    pub fn convert_board_type(&self, level: Level) -> Option<BoardType> {
        match level {
            Level::White => Some(BoardType::White),
            Level::Neutral => Some(BoardType::Neutral),
            Level::Black => Some(BoardType::Black),
            _ => None,
        }
    }

    pub fn convert_level(&self, board_type: BoardType) -> Level {
        match board_type {
            BoardType::White => Level::White,
            BoardType::Neutral => Level::Neutral,
            BoardType::Black => Level::Black,
            _ => todo!(),
        }
    }

    pub fn get_piece(&self, square: BitBoard) -> Option<&Piece> {
        self.pieces.iter().find(|piece| piece.position == square)
    }

    pub fn remove_piece(&mut self, square: BitBoard) -> Option<Piece> {
        match self
            .pieces
            .iter()
            .position(|piece| piece.position == square)
        {
            Some(index) => Some(self.pieces.remove(index)),
            None => None,
        }
    }

    pub fn set_piece(&mut self, square: BitBoard, piece: PieceType, color: Color) -> Option<Piece> {
        let old_piece = self.remove_piece(square);

        self.pieces.push(Piece::new(square, piece, color));

        old_piece
    }

    pub fn validate_square(&self, square: BitBoard) -> bool {
        let level = BitBoard::into_square(&square).level;
        let square = square.remove_level();

        match level {
            Level::White => BitBoard::WHITE_SET.contains(square),
            Level::Neutral => BitBoard::NEUTRAL_SET.contains(square),
            Level::Black => BitBoard::BLACK_SET.contains(square),
            Level::QL1 => BitBoard::QL1_SET.contains(square),
            Level::QL2 => BitBoard::QL2_SET.contains(square),
            Level::QL3 => BitBoard::QL3_SET.contains(square),
            Level::QL4 => BitBoard::QL4_SET.contains(square),
            Level::QL5 => BitBoard::QL5_SET.contains(square),
            Level::QL6 => BitBoard::QL6_SET.contains(square),
            Level::KL1 => BitBoard::KL1_SET.contains(square),
            Level::KL2 => BitBoard::KL2_SET.contains(square),
            Level::KL3 => BitBoard::KL3_SET.contains(square),
            Level::KL4 => BitBoard::KL4_SET.contains(square),
            Level::KL5 => BitBoard::KL5_SET.contains(square),
            Level::KL6 => BitBoard::KL6_SET.contains(square),
        }
    }

    pub fn update_occupied(&mut self) {
        self.occupied = ColorMask::new();

        for piece in self.pieces.iter() {
            match  self.convert_board_type(piece.position.get_level()) {
                Some(board_type) => {
                    self.occupied[piece.color][board_type] |= piece.position;
                }
                None => (),
            }
        }
    }

    pub fn update(&mut self) {
        self.update_occupied();
        
        for piece in self.pieces.iter_mut() {
           piece.update_attacks(&self.occupied);
        }
    }
}

pub struct Game {
    pub turn: Color,
    pub board: Board,
    pub move_stack: Vec<(PieceMove, BoardSnapshot)>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            turn: Color::White,
            board: Board::new(),
            move_stack: Vec::new(),
        };

        game.board
            .set_piece(BitBoard::Z0 | BitBoard::QL1, PieceType::Rook, Color::White);
        game.board
            .set_piece(BitBoard::A0 | BitBoard::QL1, PieceType::Queen, Color::White);
        game.board
            .set_piece(BitBoard::Z1 | BitBoard::QL1, PieceType::Pawn, Color::White);
        game.board
            .set_piece(BitBoard::A1 | BitBoard::QL1, PieceType::Pawn, Color::White);
        game.board.set_piece(
            BitBoard::A1 | BitBoard::WHITE,
            PieceType::Knight,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::B1 | BitBoard::WHITE,
            PieceType::Bishop,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::C1 | BitBoard::WHITE,
            PieceType::Bishop,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::D1 | BitBoard::WHITE,
            PieceType::Knight,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::A2 | BitBoard::WHITE,
            PieceType::Pawn,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::B2 | BitBoard::WHITE,
            PieceType::Pawn,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::C2 | BitBoard::WHITE,
            PieceType::Pawn,
            Color::White,
        );
        game.board.set_piece(
            BitBoard::D2 | BitBoard::WHITE,
            PieceType::Pawn,
            Color::White,
        );
        game.board
            .set_piece(BitBoard::D0 | BitBoard::KL1, PieceType::King, Color::White);
        game.board
            .set_piece(BitBoard::E0 | BitBoard::KL1, PieceType::Rook, Color::White);
        game.board
            .set_piece(BitBoard::D1 | BitBoard::KL1, PieceType::Pawn, Color::White);
        game.board
            .set_piece(BitBoard::E1 | BitBoard::KL1, PieceType::Pawn, Color::White);

        game.board
            .set_piece(BitBoard::Z8 | BitBoard::QL6, PieceType::Pawn, Color::Black);
        game.board
            .set_piece(BitBoard::A8 | BitBoard::QL6, PieceType::Pawn, Color::Black);
        game.board
            .set_piece(BitBoard::Z9 | BitBoard::QL6, PieceType::Rook, Color::Black);
        game.board
            .set_piece(BitBoard::A9 | BitBoard::QL6, PieceType::Queen, Color::Black);
        game.board.set_piece(
            BitBoard::A7 | BitBoard::BLACK,
            PieceType::Pawn,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::B7 | BitBoard::BLACK,
            PieceType::Pawn,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::C7 | BitBoard::BLACK,
            PieceType::Pawn,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::D7 | BitBoard::BLACK,
            PieceType::Pawn,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::A8 | BitBoard::BLACK,
            PieceType::Knight,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::B8 | BitBoard::BLACK,
            PieceType::Bishop,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::C8 | BitBoard::BLACK,
            PieceType::Bishop,
            Color::Black,
        );
        game.board.set_piece(
            BitBoard::D8 | BitBoard::BLACK,
            PieceType::Knight,
            Color::Black,
        );
        game.board
            .set_piece(BitBoard::D8 | BitBoard::KL6, PieceType::Pawn, Color::Black);
        game.board
            .set_piece(BitBoard::E8 | BitBoard::KL6, PieceType::Pawn, Color::Black);
        game.board
            .set_piece(BitBoard::D9 | BitBoard::KL6, PieceType::King, Color::Black);
        game.board
            .set_piece(BitBoard::E9 | BitBoard::KL6, PieceType::Rook, Color::Black);

        game
    }

    fn pass_turn(&mut self) {
        self.turn != self.turn;
    }

    pub fn legal_move(&self, piece_move: PieceMove) -> bool {
        todo!()
    }

    pub fn push_move(&mut self, piece_move: PieceMove) -> Result<(), &'static str> {
        let snapshot = BoardSnapshot::new(&self.board);

        let source = BitBoard::from_square(&piece_move.source);
        let destination = BitBoard::from_square(&piece_move.destination);

        let piece = self
            .board
            .remove_piece(source)
            .ok_or("There is no piece at the source")?;

        let captured_piece = self
            .board
            .set_piece(destination, piece.piece_type, piece.color);

        match captured_piece {
            Some(piece) => self.board.captured_pieces.push(piece),
            None => (),
        }

        self.move_stack.push((piece_move, snapshot));
        self.pass_turn();

        Ok(())
    }

    pub fn pop_move(&mut self) -> Result<PieceMove, &'static str> {
        self.pass_turn();
        match self.move_stack.pop() {
            Some((piece_move, snapshot)) => {
                snapshot.restore(&mut self.board);
                Ok(piece_move)
            }
            None => Err("Nothing to pop"),
        }
    }

    pub fn print(&self) {
        println!("White Board: ");
        for bit_square in BitBoard::WHITE_SET.iter() {
            match self.board.get_piece(bit_square | BitBoard::WHITE) {
                Some(piece) => print!("{} ", piece.get_char()),
                None => print!(". "),
            }

            if bit_square.get_rank() == Rank::Four {
                println!();
            }
        }

        println!("Neutral Board: ");
        for bit_square in BitBoard::NEUTRAL_SET.iter() {
            match self.board.get_piece(bit_square | BitBoard::NEUTRAL) {
                Some(piece) => print!("{} ", piece.get_char()),
                None => print!(". "),
            }

            if bit_square.get_rank() == Rank::Six {
                println!();
            }
        }

        println!("Black Board: ");
        for bit_square in BitBoard::BLACK_SET.iter() {
            match self.board.get_piece(bit_square | BitBoard::BLACK) {
                Some(piece) => print!("{} ", piece.get_char()),
                None => print!(". "),
            }

            if bit_square.get_rank() == Rank::Eight {
                println!();
            }
        }
    }
}
