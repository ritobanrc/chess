use crate::piece::{Piece, PieceData, Side};
use crate::BOARD_SIZE;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
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
        captured: Piece,
    },
}

pub struct Chessboard {
    pub pieces: HashMap<[u8; 2], Piece>,
    pub en_passant: Option<[u8; 2]>,
}

pub fn create_piece(
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
    [s[0] - b'A', s[1] - b'1']
}

impl Chessboard {
    /// Creates an empty chessboard with no pieces on it.
    pub fn empty() -> Chessboard {
        Chessboard {
            pieces: HashMap::new(),
            en_passant: None,
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
            pieces,
            en_passant: None,
        }
    }

    pub fn on_board(pos: [i8; 2]) -> Option<[u8; 2]> {
        if pos[0] >= 0 && pos[1] >= 0 && pos[0] < BOARD_SIZE as i8 && pos[1] < BOARD_SIZE as i8 {
            return Some([pos[0] as u8, pos[1] as u8]);
        }
        None
    }

    #[inline(always)]
    pub fn get_piece_at(&self, pos: [u8; 2]) -> Option<&Piece> {
        self.pieces.get(&pos)
    }

    #[inline(always)]
    pub fn get_pieces(&self) -> &HashMap<[u8; 2], Piece> {
        &self.pieces
    }

    /// A wrapper on HashMap::insert, which just inserts the piece into the hashmap without any
    /// checks, and returns the piece it might have replaced.
    pub fn apply_move(&mut self, pos: [u8; 2], piece: Piece) -> Option<Piece> {
        self.pieces.insert(pos, piece)
    }

    pub fn try_move(&mut self, piece_ref: &Piece, end_pos: [u8; 2]) -> MoveResult {
        // get the copy in the hashset. We can't be certain that piece_ref references the hashset.
        let piece = self.pieces.remove(&piece_ref.get_data().position).unwrap();

        piece.try_move(self, end_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_chessboard() {
        let chessboard = Chessboard::empty();
        assert_eq!(chessboard.get_piece_at([0, 0]), None);
    }

    #[test]
    fn test_piece_creation() {
        let mut chessboard = Chessboard::empty();
        create_piece(
            &mut chessboard.pieces,
            str_to_pos("e5"),
            Side::Light,
            &Piece::Rook,
        );
        assert_eq!(
            chessboard.get_piece_at([4, 4]).unwrap(),
            &Piece::Rook(PieceData {
                position: [4, 4],
                side: Side::Light
            })
        );
        assert_eq!(chessboard.get_piece_at([5, 5]), None);
    }

}
