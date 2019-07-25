use std::collections::HashMap;
use crate::piece::{Piece, PieceData, Side};


pub struct Chessboard {
    pieces: HashMap<[u8; 2], Piece>,
}


fn create_piece(pieces: &mut HashMap<[u8; 2], Piece>,
                pos: [u8; 2],
                side: Side,
                piece_type: &Fn(PieceData) -> Piece) { 
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

        create_piece(&mut pieces, str_to_pos("a1"), Side::Light, &Piece::Rook);
        create_piece(&mut pieces, str_to_pos("b1"), Side::Light, &Piece::Knight);
        create_piece(&mut pieces, str_to_pos("c1"), Side::Light, &Piece::Bishop);
        create_piece(&mut pieces, str_to_pos("d1"), Side::Light, &Piece::Queen);
        create_piece(&mut pieces, str_to_pos("e1"), Side::Light, &Piece::King);
        create_piece(&mut pieces, str_to_pos("f1"), Side::Light, &Piece::Bishop);
        create_piece(&mut pieces, str_to_pos("g1"), Side::Light, &Piece::Knight);
        create_piece(&mut pieces, str_to_pos("h1"), Side::Light, &Piece::Rook);

        create_piece(&mut pieces, str_to_pos("a8"), Side::Dark, &Piece::Rook);
        create_piece(&mut pieces, str_to_pos("b8"), Side::Dark, &Piece::Knight);
        create_piece(&mut pieces, str_to_pos("c8"), Side::Dark, &Piece::Bishop);
        create_piece(&mut pieces, str_to_pos("d8"), Side::Dark, &Piece::Queen);
        create_piece(&mut pieces, str_to_pos("e8"), Side::Dark, &Piece::King);
        create_piece(&mut pieces, str_to_pos("f8"), Side::Dark, &Piece::Bishop);
        create_piece(&mut pieces, str_to_pos("g8"), Side::Dark, &Piece::Knight);
        create_piece(&mut pieces, str_to_pos("h8"), Side::Dark, &Piece::Rook);



        // Create pawns
        for file in 0..8u8 {
            create_piece(&mut pieces, [file, 1], Side::Light, &Piece::Pawn);
            create_piece(&mut pieces, [file, 6], Side::Dark, &Piece::Pawn);
        }

        Chessboard {
            pieces: pieces,
        }
    }

    pub fn get_piece_at(&self, pos: [u8; 2]) -> Option<&Piece> {
        self.pieces.get(&pos)
    }

    pub fn get_pieces(&self) -> &HashMap<[u8; 2], Piece> {
        &self.pieces
    }
}

