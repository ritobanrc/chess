pub trait Piece {
    fn get_position(&self) -> [u8; 2];
}

pub struct Pawn {
    position: [u8; 2]
}

impl Piece for Pawn {
    fn get_position(&self) -> [u8; 2] {
        self.position
    }
}
