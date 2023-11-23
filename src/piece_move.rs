use crate::{square::Square, piece::Piece};

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct PieceMove {
    pub source: Square,
    pub destination: Square,
    pub promotion: Option<Piece>,
}

impl PieceMove {
    fn new(source: Square, destination: Square, promotion: Option<Piece>) -> Self {
        Self {
            source,
            destination,
            promotion,
        }
    }
}
