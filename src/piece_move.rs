use crate::{piece::PieceType, square::{Square, BitBoard}};

pub struct PieceMove {
    pub source: Square,
    pub dest: Square,
    pub promotion: Option<PieceType>,
}

impl PieceMove {
    fn new(source: Square, dest: Square, promotion: Option<PieceType>) -> Self {
        Self {
            source,
            dest,
            promotion,
        }
    }
}
