#[derive(PartialEq)]
pub enum Side { Light, Dark }

pub struct PieceData {
    pub position: [u8; 2], 
    pub side: Side,
}

impl PieceData {
    pub fn new (position: [u8; 2], side: Side) -> PieceData {
        PieceData {
            position: position, 
            side: side,
        }
    }
}

pub enum Piece {
    Pawn(PieceData), 
    Rook(PieceData), 
    Knight(PieceData),
    Bishop(PieceData),
    Queen(PieceData), 
    King(PieceData),
}
