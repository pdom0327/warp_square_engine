use crate::{
    piece::{PieceType, Piece},
    // color_mask::ColorMask,
    piece_move::PieceMove,
    square::{Color, Level, Rank, Square}, color_mask::ColorMask, bit_board::{BitBoardSet, BitBoard, BoardType},
};

pub struct BoardSnapshot {
    pub occupied_mask: ColorMask,
    pub pawns: BitBoardSet,
    pub knights: BitBoardSet,
    pub bishops: BitBoardSet,
    pub rooks: BitBoardSet,
    pub queens: BitBoardSet,
    pub kings: BitBoardSet,
}

impl BoardSnapshot {
    pub fn new(board: &Board) -> Self {
        Self {
            occupied_mask: board.occupied_mask.clone(),
            pawns: board.pawns.clone(),
            knights: board.knights.clone(),
            bishops: board.bishops.clone(),
            rooks: board.rooks.clone(),
            queens: board.queens.clone(),
            kings: board.kings.clone(),
        }
    }

    pub fn restore(&self, board: &mut Board) {
        board.occupied_mask = self.occupied_mask;
        board.pawns = self.pawns;
        board.knights = self.knights;
        board.bishops = self.bishops;
        board.rooks = self.rooks;
        board.queens = self.queens;
        board.kings = self.kings;
    }
}

pub struct Board {
    white_pieces: Vec<Piece>,
    black_pieces: Vec<Piece>
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_pieces: Vec::new(),
            black_pieces: Vec::new(),
        }
    }

    pub fn set_piece(&mut self, square: &Square, piece: PieceType, color: Color) {
        let board_type = match square {
            Square { level: Level::White, .. } => BoardType::White,
            Square { level: Level::Neutral, .. } => BoardType::Neutral,
            Square { level: Level::Black, .. } => BoardType::Black,
            _ => BoardType::Attack
        };

        let bit_square = BitBoard::from_square(square);

        let bit_board = match piece {
            PieceType::Pawn => self.pawns[board_type],
            PieceType::Knight => self.knights[board_type],
            PieceType::Bishop => self.bishops[board_type],
            PieceType::Rook => self.rooks[board_type],
            PieceType::Queen => self.queens[board_type],
            PieceType::King => self.kings[board_type],
        };

        self.occupied_mask[color][board_type] |= bit_square;
    }
}

pub struct Game {
    turn: Color,
    board: Board,
    move_stack: Vec<(PieceMove, BoardSnapshot)>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            turn: Color::White,
            board: Board::new(),
            move_stack: Vec::new(),
        };

        todo!();

        game
    }

    fn pass_turn(&mut self) {
        self.turn != self.turn;
    }

    pub fn legal_move(&self, piece_move: PieceMove) -> bool {
        todo!()
    }

    pub fn push_move(&mut self, piece_move: PieceMove) -> Result<Option<PieceType>, ()> {
        let snapshot = BoardSnapshot::new(&self.board);

        let captured_piece = match self.pieces.remove(&piece_move.source) {
            Some(piece) => {
                self.pieces.insert(piece_move.destination.clone(), piece)
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

    pub fn print(&self) {
        println!("White Board: ");
        for (rank, file) in Self::WHITE_SET {
            let square = Square::new(rank, file, Level::White);

            match self.pieces.get(&square) {
                Some(piece) => print!("{} ", piece),
                None => print!(". "),
            }

            if rank == Rank::Four {
                println!();
            }
        }

        println!("Neutral Board: ");
        for (rank, file) in Self::NEUTRAL_SET {
            let square = Square::new(rank, file, Level::Neutral);

            match self.pieces.get(&square) {
                Some(piece) => print!("{} ", piece),
                None => print!(". "),
            }

            if rank == Rank::Six {
                println!();
            }
        }

        println!("Black Board: ");
        for (rank, file) in Self::BLACK_SET {
            let square = Square::new(rank, file, Level::Black);

            match self.pieces.get(&square) {
                Some(piece) => print!("{} ", piece.get_char(color)),
                None => print!(". "),
            }

            if rank == Rank::Eight {
                println!();
            }
        }
    }
}
