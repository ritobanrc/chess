use std::rc::Rc;
pub use crate::piece::Piece;

pub struct Chessboard {
    board: [[Rc<dyn Piece>; 8]; 8]
}

impl Chessboard {
    fn get_piece_at(&self, pos: [usize; 2]) -> Rc<dyn Piece> {
        self.board[pos[0]][pos[1]].clone()
    }
}

