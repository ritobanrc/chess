use crate::piece::{Piece, PieceData, Side, MoveType};
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

#[derive(Clone)]
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
    fn insert(&mut self, pos: [u8; 2], piece: Piece) -> Option<Piece> {
        self.pieces.insert(pos, piece)
    }

    pub fn apply_move(&mut self, mut piece: Piece, move_type: MoveType, end_pos: [u8; 2]) -> MoveResult {
        match move_type {
            MoveType::Invalid => {
                self.insert(piece.get_data().position, piece);
                self.en_passant = None;
                MoveResult::Invalid
            },

            MoveType::Capture => {
                piece.get_data_mut().position = end_pos;
                self.en_passant = None;
                let captured = self.insert(end_pos, piece).unwrap();
                MoveResult::Capture {
                    moved: self.get_piece_at(end_pos).unwrap(),
                    captured,
                }
            },

            MoveType::Regular => {
                piece.get_data_mut().position = end_pos;
                self.en_passant = None;
                self.insert(end_pos, piece);
                MoveResult::Regular(self.get_piece_at(end_pos).unwrap())
            },

            MoveType::Doublestep => {
                piece.get_data_mut().position = end_pos;
                self.en_passant = Some(piece.get_data().position);
                self.insert(end_pos, piece);
                MoveResult::Regular(self.get_piece_at(end_pos).unwrap())
            },

            MoveType::EnPassant => { 
                piece.get_data_mut().position = end_pos;
                self.insert(end_pos, piece); // this should not return anything
                let captured = self.pieces.remove(&self.en_passant.unwrap()).unwrap();
                self.en_passant = None;
                MoveResult::EnPassant {
                    moved: self.get_piece_at(end_pos).unwrap(),
                    captured,
                }
            }
        }
    }

    pub fn try_move(&mut self, piece_ref: &Piece, end_pos: [u8; 2]) -> MoveResult {
        // get the copy in the hashset. We can't be certain that piece_ref references the hashset.
        let piece = self.pieces.remove(&piece_ref.get_data().position).unwrap();

        let move_type = piece.can_move(self, end_pos, true);

        self.apply_move(piece, move_type, end_pos)
        //piece.try_move(self, end_pos)
    }

    pub fn get_king(&self, side: Side) -> Option<&Piece> {
        for (_pos, piece) in self.pieces.iter() {
            if let Piece::King(data) = piece {
                if data.side == side {
                    return Some(piece)
                }
            }
        }
        None
    }

    pub fn is_in_check(&self, side: Side) -> bool {
        let king = self.get_king(side).unwrap();
        for (_pos, piece) in self.pieces.iter() {
            if piece.get_data().side == king.get_data().side {
                continue;
            }
            match piece {
                Piece::Pawn(_data)
                | Piece::Rook(_data)
                | Piece::Knight(_data)
                | Piece::Bishop(_data)
                | Piece::Queen(_data)
                | Piece::King(_data) => {
                    if let MoveType::Capture = piece.can_move(self, king.get_data().position, false) {
                        return true
                    }
                },
                //Piece::King(data) => {
                    //let (dx, dy) = Piece::get_dx_dy(data.position, king.get_data().position);
                    //if dx.abs() <= 1 && dy.abs() <= 1 {
                        //return true
                    //} 
                //}
            }
        }
        false
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
