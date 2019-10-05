use crate::chessboard::Chessboard;
use crate::piece::Piece;
use std::{thread, time};

pub fn get_best_move(chessboard: &Chessboard) -> (&Piece, [u8; 2]) {
    //return (chessboard.values().iter().next(), [0, 0])
    //thread::sleep(time::Duration::new(2, 0));
    chessboard.possible_moves(chessboard.turn)[0]
}
