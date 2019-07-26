use std::hash::{Hash, Hasher};

#[derive(PartialEq, Hash, Debug)]
pub enum Side {
    Light,
    Dark,
}

#[derive(PartialEq, Debug)]
pub struct PieceData {
    pub position: [u8; 2],
    pub side: Side,
}

impl PieceData {
    pub fn new(position: [u8; 2], side: Side) -> PieceData {
        PieceData { position, side }
    }
}

//impl PartialEq for PieceData {
//fn eq(&self, other: &Self) -> bool {
//self.position == other.position && self.side == other.side
//}
//}

impl Eq for Piece {}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.get_data() == other.get_data()
    }
}

impl Hash for Piece {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get_data().position.hash(state);
        self.get_data().side.hash(state);
    }
}

impl Piece {
    pub fn get_data(&self) -> &PieceData {
        match &self {
            Piece::Pawn(data)
            | Piece::Rook(data)
            | Piece::Knight(data)
            | Piece::Bishop(data)
            | Piece::Queen(data)
            | Piece::King(data) => &data,
        }
    }

    pub fn get_data_mut(&mut self) -> &mut PieceData {
        match self {
            Piece::Pawn(data)
            | Piece::Rook(data)
            | Piece::Knight(data)
            | Piece::Bishop(data)
            | Piece::Queen(data)
            | Piece::King(data) => data,
        }
    }
}

#[derive(Debug)]
pub enum Piece {
    Pawn(PieceData),
    Rook(PieceData),
    Knight(PieceData),
    Bishop(PieceData),
    Queen(PieceData),
    King(PieceData),
}
