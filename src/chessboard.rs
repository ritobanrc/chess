use crate::piece::{Piece, PieceData, Side};
use std::collections::HashMap;

pub enum MoveResult<'a> {
    Invalid,
    Regular(&'a Piece), 
    Capture {
        moved: &'a Piece,
        captured: Piece, // capturing a piece gives up ownership
    },
    Castle {
        king: &'a Piece, 
        rook: &'a Piece,
    }, 
    EnPassant {
        moved: &'a Piece,
        captured: &'a Piece,
    }
}

pub struct Chessboard {
    pub pieces: HashMap<[u8; 2], Piece>,
}

fn create_piece(
    pieces: &mut HashMap<[u8; 2], Piece>,
    pos: [u8; 2],
    side: Side,
    piece_type: &Fn(PieceData) -> Piece,
) {
    let data = PieceData::new(pos, side);
    pieces.insert(pos, piece_type(data));
}

fn str_to_pos(s: &str) -> [u8; 2] {
    let s = s.to_ascii_uppercase();
    let s = s.as_bytes();
    [s[0] - b'A' as u8, s[1] - b'1' as u8]
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

        Chessboard { pieces }
    }

    #[inline(always)]
    pub fn get_piece_at(&self, pos: [u8; 2]) -> Option<&Piece> {
        self.pieces.get(&pos)
    }

    #[inline(always)]
    pub fn get_pieces(&self) -> &HashMap<[u8; 2], Piece> {
        &self.pieces
    }

    pub fn try_move(&mut self, piece_ref: &Piece, end_pos: [u8; 2]) -> MoveResult {
        // get the copy in the hashset. We can't be certain that piece_ref references the hashset.
        let mut piece = self.pieces.remove(&piece_ref.get_data().position).unwrap();
        if let Some(other_piece) = self.pieces.get(&end_pos) {
            if other_piece.get_data().side == piece.get_data().side {
                self.pieces.insert(piece.get_data().position, piece);
                MoveResult::Invalid
            }
            else {
                piece.get_data_mut().position = end_pos;
                let captured = self.pieces.insert(end_pos, piece); // captured piece will automatically be removed.
                MoveResult::Capture {
                    moved: self.get_piece_at(end_pos).unwrap(),
                    captured: captured.unwrap()
                }
            }
        }
        else {
            piece.get_data_mut().position = end_pos;
            self.pieces.insert(end_pos, piece);
            MoveResult::Regular(self.get_piece_at(end_pos).unwrap())
        }
    }
}
