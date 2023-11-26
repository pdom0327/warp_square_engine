use crate::jni_c_header::*;
use ::warp_square_engine::{
    bit_board::BitBoard,
    game::Game,
    piece::{Piece, PieceType},
    piece_move::PieceMove,
    square::{Color, File, Level, Rank, Square},
};

foreign_enum!(
    enum PieceType {
        Pawn = PieceType::Pawn,
        Knight = PieceType::Knight,
        Bishop = PieceType::Bishop,
        Rook = PieceType::Rook,
        Queen = PieceType::Queen,
        King = PieceType::King,
    }
);

foreign_enum!(
    enum Rank {
        Zero = Rank::Zero,
        One = Rank::One,
        Two = Rank::Two,
        Three = Rank::Three,
        Four = Rank::Four,
        Five = Rank::Five,
        Six = Rank::Six,
        Seven = Rank::Seven,
        Eight = Rank::Eight,
        Nine = Rank::Nine,
    }
);

foreign_enum!(
    enum File {
        Z = File::Z,
        A = File::A,
        B = File::B,
        C = File::C,
        D = File::D,
        E = File::E,
    }
);

foreign_enum!(
    enum Level {
        White = Level::White,
        Neutral = Level::Neutral,
        Black = Level::Black,
        QL1 = Level::QL1,
        QL2 = Level::QL2,
        QL3 = Level::QL3,
        QL4 = Level::QL4,
        QL5 = Level::QL5,
        QL6 = Level::QL6,
        KL1 = Level::KL1,
        KL2 = Level::KL2,
        KL3 = Level::KL3,
        KL4 = Level::KL4,
        KL5 = Level::KL5,
        KL6 = Level::KL6,
    }
);

foreign_enum!(
    enum Color {
        White = Color::White,
        Black = Color::Black,
    }
);

foreign_class!(class Piece {
    self_type Piece;
    private constructor = empty;
    fn Piece::getPieceType(&self) -> PieceType {
        this.piece_type
    }
    fn Piece::getColor(&self) -> Color {
        this.color
    }
    fn Piece::get_square(&self) -> Square;
    fn Piece::get_char(&self) -> &'static str; alias getChar;
    foreign_code r#"
    static {
        try {
            NativeUtils.loadLibraryFromJar();
        } catch (java.io.IOException e) {
            e.printStackTrace();
        }
    }
"#;
});

foreign_class!(class PieceMove {
    self_type PieceMove;
    constructor PieceMove::new(source: Square, destination: Square, promotion: Option<PieceType>) -> PieceMove;
    foreign_code r#"
    static {
        try {
            NativeUtils.loadLibraryFromJar();
        } catch (java.io.IOException e) {
            e.printStackTrace();
        }
    }
"#;
});

foreign_class!(class Square {
    self_type Square;
    constructor Square::new(rank: Rank, file: File, level: Level) -> Square;
    foreign_code r#"
    static {
        try {
            NativeUtils.loadLibraryFromJar();
        } catch (java.io.IOException e) {
            e.printStackTrace();
        }
    }
"#;
});

foreign_class!(class Game {
    self_type Game;
    constructor Game::new() -> Game;
    fn Game::legal_move(&self, _: PieceMove) -> bool; alias legalMove;
    fn Game::push_move(&mut self, _: PieceMove) -> Result<(), &'static str>; alias pushMove;
    fn Game::pop_move(&mut self) -> Result<PieceMove, &'static str>; alias popMove;
    fn Game::print(&self);
    fn Game::getPieces(&self) -> Vec<Piece> {
        this.board.pieces.clone()
    }
    fn Game::getCapturedPieces(&self) -> Vec<Piece> {
        this.board.captured_pieces.clone()
    }
    foreign_code r#"
    static {
        try {
            NativeUtils.loadLibraryFromJar();
        } catch (java.io.IOException e) {
            e.printStackTrace();
        }
    }
"#;
});
