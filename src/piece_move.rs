use crate::{square::Square, piece::PieceType};

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug, Hash)]
pub struct PieceMove {
    pub source: Square,
    pub destination: Square,
    pub promotion: Option<PieceType>,
}

impl PieceMove {
    pub fn new(source: Square, destination: Square, promotion: Option<PieceType>) -> Self {
        Self {
            source,
            destination,
            promotion,
        }
    }
}
