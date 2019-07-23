use std::collections::HashMap;
use crate::piece::{Piece, PieceData, Side};


pub struct Chessboard {
    pieces: HashMap<[u8; 2], Piece>,
}


fn create_piece(pieces: &mut HashMap<[u8; 2], Piece>,
                pos: &'static str,
                side: Side,
                piece_type: &Fn(PieceData) -> Piece) { 
    let pos =  str_to_pos(pos);
    let data = PieceData::new(pos, side);
    pieces.insert(pos, piece_type(data));
}

fn str_to_pos (s: &'static str) -> [u8; 2]{
    let s = s.to_ascii_uppercase();
    let s = s.as_bytes();
    [s[0] - 'A' as u8, s[1] - '1' as u8]
}

impl Chessboard {
    /// Creates an empty chessboard with no pieces on it.
    pub fn empty() -> Chessboard {
        Chessboard {
            pieces: HashMap::new(),
        }
    }


    /// Creates a new chessboard with the standard arrangement of pieces
    pub fn standard() -> Chessboard {
        let mut pieces = HashMap::new();

        create_piece(&mut pieces, "a1", Side::Light, &Piece::Rook);
        create_piece(&mut pieces, "b1", Side::Light, &Piece::Knight);
        create_piece(&mut pieces, "c1", Side::Light, &Piece::Bishop);
        create_piece(&mut pieces, "d1", Side::Light, &Piece::Queen);
        create_piece(&mut pieces, "e1", Side::Light, &Piece::King);
        create_piece(&mut pieces, "f1", Side::Light, &Piece::Bishop);
        create_piece(&mut pieces, "g1", Side::Light, &Piece::Knight);
        create_piece(&mut pieces, "h1", Side::Light, &Piece::Rook);

        create_piece(&mut pieces, "a8", Side::Dark, &Piece::Rook);
        create_piece(&mut pieces, "b8", Side::Dark, &Piece::Knight);
        create_piece(&mut pieces, "c8", Side::Dark, &Piece::Bishop);
        create_piece(&mut pieces, "d8", Side::Dark, &Piece::Queen);
        create_piece(&mut pieces, "e8", Side::Dark, &Piece::King);
        create_piece(&mut pieces, "f8", Side::Dark, &Piece::Bishop);
        create_piece(&mut pieces, "g8", Side::Dark, &Piece::Knight);
        create_piece(&mut pieces, "h8", Side::Dark, &Piece::Rook);

        Chessboard {
            pieces: pieces,
        }
    }

    pub fn get_piece_at(&self, pos: &'static str) -> Option<&Piece> {
        self.pieces.get(&str_to_pos(pos))
    }
}

