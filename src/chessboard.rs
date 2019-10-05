use crate::piece::{MoveType, Piece, PieceData, Side};
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
        rook_init_pos: [u8; 2],
    },
    EnPassant {
        moved: &'a Piece,
        captured: Piece,
    },
    PawnPromotion(&'a Piece),
    PawnPromotionCapture {
        moved: &'a Piece,
        captured: Piece, // capturing a piece gives up ownership
    },
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CastleRights {
    NoRights,
    KingSide,
    QueenSide,
    Both,
}

impl CastleRights {
    /// Returns NoRights if the castle is invalid,
    /// or KingSide or Queenside if if the castle is valid.
    /// Will not return Both
    pub fn check_end_pos(self, end_pos: [u8; 2], side: Side) -> CastleRights {
        if side.back_rank() != end_pos[1] {
            CastleRights::NoRights
        } else {
            match self {
                CastleRights::NoRights => CastleRights::NoRights,
                CastleRights::KingSide => {
                    if end_pos[0] == 6u8 {
                        CastleRights::KingSide
                    } else {
                        CastleRights::NoRights
                    }
                }
                CastleRights::QueenSide => {
                    if end_pos[0] == 2u8 {
                        CastleRights::QueenSide
                    } else {
                        CastleRights::NoRights
                    }
                }
                CastleRights::Both => {
                    if end_pos[0] == 6u8 {
                        CastleRights::KingSide
                    } else if end_pos[0] == 2u8 {
                        CastleRights::QueenSide
                    } else {
                        CastleRights::NoRights
                    }
                }
            }
        }
    }

    pub fn remove_rights(self, to_remove: CastleRights) -> CastleRights {
        if self == CastleRights::NoRights || to_remove == CastleRights::NoRights {
            return self; // nothing changes
        }
        if to_remove == CastleRights::Both || self == to_remove {
            return CastleRights::NoRights; // if both are removed, or what we remove is all we had, nothing is left.
        }
        if self == CastleRights::Both {
            match to_remove {
                CastleRights::QueenSide => return CastleRights::KingSide,
                CastleRights::KingSide => return CastleRights::QueenSide,
                _ => {}
            }
        }
        self // only if self and to_remove are mutually exclusive, so nothing changes.
    }

    pub fn rook_final_pos(self, side: Side) -> Result<[u8; 2], String> {
        match self {
            CastleRights::NoRights | CastleRights::Both => Err(format!(
                "CastleRights::rook_final_pos -- {:?} is not a valid input.",
                self
            )),
            CastleRights::KingSide => Ok([BOARD_SIZE - 3, side.back_rank()]),
            CastleRights::QueenSide => Ok([3, side.back_rank()]),
        }
    }

    pub fn rook_init_pos(self, side: Side) -> Result<[u8; 2], String> {
        match self {
            CastleRights::NoRights | CastleRights::Both => Err(format!(
                "CastleRights::rook_init_pos -- {:?} is not a valid input.",
                self
            )),
            CastleRights::KingSide => Ok([BOARD_SIZE - 1, side.back_rank()]),
            CastleRights::QueenSide => Ok([0, side.back_rank()]),
        }
    }

    pub fn castle_rights_for_rook(piece: &Piece) -> Result<CastleRights, &str> {
        let pos = piece.data().position;
        if pos[1] != piece.data().side.back_rank() {
            Err("Rook not on back rank.")
        } else if pos[0] == 0 {
            Ok(CastleRights::QueenSide)
        } else if pos[0] == BOARD_SIZE - 1 {
            Ok(CastleRights::KingSide)
        } else {
            Err("Rook not where expected.")
        }
    }
}

#[derive(Clone)]
pub struct Chessboard {
    pub pieces: HashMap<[u8; 2], Piece>,
    pub en_passant: Option<[u8; 2]>,
    pub turn: Side,
    light_castle: CastleRights,
    dark_castle: CastleRights,
}

pub fn create_piece(
    pieces: &mut HashMap<[u8; 2], Piece>,
    pos: [u8; 2],
    side: Side,
    piece_type: &dyn Fn(PieceData) -> Piece,
) {
    let data = PieceData::new(pos, side);
    pieces.insert(pos, piece_type(data));
}

fn str_to_pos(s: &str) -> [u8; 2] {
    let s = s.to_ascii_uppercase();
    let s = s.as_bytes();
    [s[0] - b'A', s[1] - b'1']
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Checkmate {
    Nothing,
    Checkmate,
    Stalemate,
}

impl Chessboard {
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
            turn: Side::Light,
            light_castle: CastleRights::Both,
            dark_castle: CastleRights::Both,
        }
    }

    pub fn castle_rights(&self, side: Side) -> CastleRights {
        match side {
            Side::Light => self.light_castle,
            Side::Dark => self.dark_castle,
        }
    }

    pub fn on_board(pos: [i8; 2]) -> Option<[u8; 2]> {
        if pos[0] >= 0 && pos[1] >= 0 && pos[0] < BOARD_SIZE as i8 && pos[1] < BOARD_SIZE as i8 {
            return Some([pos[0] as u8, pos[1] as u8]);
        }
        None
    }

    #[inline(always)]
    pub fn piece_at(&self, pos: [u8; 2]) -> Option<&Piece> {
        self.pieces.get(&pos)
    }

    #[inline(always)]
    pub fn pieces(&self) -> &HashMap<[u8; 2], Piece> {
        &self.pieces
    }

    /// A wrapper on HashMap::insert, which just inserts the piece into the hashmap without any
    /// checks, and returns the piece it might have replaced.
    pub fn insert(&mut self, pos: [u8; 2], piece: Piece) -> Option<Piece> {
        self.pieces.insert(pos, piece)
    }

    fn remove_all_castle_rights(&mut self, side: Side) {
        match side {
            Side::Light => self.light_castle = self.light_castle.remove_rights(CastleRights::Both),
            Side::Dark => self.dark_castle = self.dark_castle.remove_rights(CastleRights::Both),
        }
    }

    fn set_castle_rights(&mut self, side: Side, rights: CastleRights) {
        match side {
            Side::Light => self.light_castle = rights,
            Side::Dark => self.dark_castle = rights,
        }
    }

    pub fn is_checkmated(&self, side: Side) -> Checkmate {
        //println!("{:?}", side);
        for piece in self.pieces.values() {
            if piece.data().side != side {
                continue;
            }
            for i in 0..8 {
                for j in 0..8 {
                    // I'm making the assumption that it doesn't matter what you promote to.
                    // If you're going to get out of check by pawn promotion, you're either
                    //     - capturing the checking piece
                    //     - blocking
                    //  In either scenario, it doesn't matter what you promote to
                    let move_type = piece.can_move(self, [i, j], true, Some(&Piece::Queen));
                    if move_type != MoveType::Invalid {
                        //println!("{:?} to {:?} is {:?}", piece, [i, j], move_type);
                        return Checkmate::Nothing;
                    }
                }
            }
        }
        if self.is_side_in_check(side) {
            Checkmate::Checkmate
        } else {
            Checkmate::Stalemate
        }
    }

    pub fn apply_move(
        &mut self,
        mut piece: Piece,
        move_type: MoveType,
        end_pos: [u8; 2],
        promotion: Option<&dyn Fn(PieceData) -> Piece>,
    ) -> MoveResult {
        // this function used to work on the assumption that the piece was removed from the pieces
        // map, and that this function's responsibility was to add it back in. However, when we are
        // looking at checkmate, that's not the case. So, enter, this ugly hack
        self.pieces.remove(&piece.data().position); // if this is None, that means it was already handled. Otherwise, rmeove it.
        match move_type {
            MoveType::Invalid => {
                self.insert(piece.data().position, piece);
                MoveResult::Invalid
            }

            MoveType::Capture => {
                match &piece {
                    Piece::King(data) => self.remove_all_castle_rights(data.side),
                    Piece::Rook(data) => {
                        let to_remove = CastleRights::castle_rights_for_rook(&piece);
                        // if it returned an error, don't worry about it.
                        // it probably means that it's already been dealt with earlier.
                        //println!("Removing Castle Rights {:?}", to_remove);
                        if let Ok(rights) = to_remove {
                            self.set_castle_rights(
                                data.side,
                                self.castle_rights(data.side).remove_rights(rights),
                            );
                        }
                    }
                    _ => {}
                };
                piece.data_mut().position = end_pos;
                let captured = self.insert(end_pos, piece).unwrap();
                self.en_passant = None;
                self.turn = self.turn.other();
                MoveResult::Capture {
                    moved: self.piece_at(end_pos).unwrap(),
                    captured,
                }
            }

            MoveType::Regular => {
                match &piece {
                    Piece::King(data) => self.remove_all_castle_rights(data.side),
                    Piece::Rook(data) => {
                        let to_remove = CastleRights::castle_rights_for_rook(&piece);
                        // if it returned an error, don't worry about it.
                        // it probably means that it's already been dealt with earlier.
                        //println!("Removing Castle Rights {:?}", to_remove);
                        if let Ok(rights) = to_remove {
                            self.set_castle_rights(
                                data.side,
                                self.castle_rights(data.side).remove_rights(rights),
                            );
                        }
                    }
                    _ => {}
                };
                piece.data_mut().position = end_pos;
                self.en_passant = None;
                self.turn = self.turn.other();
                self.insert(end_pos, piece);
                MoveResult::Regular(self.piece_at(end_pos).unwrap())
            }

            MoveType::Doublestep => {
                piece.data_mut().position = end_pos;
                self.en_passant = Some(piece.data().position);
                self.turn = self.turn.other();
                self.insert(end_pos, piece);
                MoveResult::Regular(self.piece_at(end_pos).unwrap())
            }

            MoveType::EnPassant => {
                piece.data_mut().position = end_pos;
                self.insert(end_pos, piece); // this should not return anything
                let captured = self.pieces.remove(&self.en_passant.unwrap()).unwrap();
                self.en_passant = None;
                self.turn = self.turn.other();
                MoveResult::EnPassant {
                    moved: self.piece_at(end_pos).unwrap(),
                    captured,
                }
            }

            MoveType::Castle => {
                let castle_type = self
                    .castle_rights(piece.data().side)
                    .check_end_pos(end_pos, piece.data().side);
                let rook_pos = castle_type
                    .rook_init_pos(piece.data().side)
                    .unwrap();
                // move the king
                piece.data_mut().position = end_pos;

                let mut rook = self.pieces.remove(&rook_pos).unwrap();
                let rook_end_pos = castle_type
                    .rook_final_pos(piece.data().side)
                    .unwrap();
                rook.data_mut().position = rook_end_pos;
                self.en_passant = None;
                // we can only castle once
                self.remove_all_castle_rights(piece.data().side);
                self.insert(rook_end_pos, rook);
                self.insert(end_pos, piece);
                self.turn = self.turn.other();
                MoveResult::Castle {
                    king: self.piece_at(end_pos).unwrap(),
                    rook: self.piece_at(rook_end_pos).unwrap(),
                    rook_init_pos: rook_pos,
                }
                //MoveResult::Regular(self.piece_at(end_pos).unwrap())
            }

            MoveType::PawnPromotion => {
                if let Piece::Pawn(data) = piece {
                    piece = promotion.expect("Chessboard::apply_move -- pawn promotion Fn is none")(
                        data,
                    );
                }
                piece.data_mut().position = end_pos;
                self.en_passant = None;
                self.insert(end_pos, piece);
                self.turn = self.turn.other();
                MoveResult::PawnPromotion(self.piece_at(end_pos).unwrap())
            }

            MoveType::PawnPromotionCapture => {
                if let Piece::Pawn(data) = piece {
                    piece = promotion.expect("Chessboard::apply_move -- pawn promotion Fn is none")(
                        data,
                    );
                }
                piece.data_mut().position = end_pos;
                let captured = self.insert(end_pos, piece).unwrap();
                self.en_passant = None;
                self.turn = self.turn.other();
                MoveResult::PawnPromotionCapture {
                    moved: self.piece_at(end_pos).unwrap(),
                    captured,
                }
            }
        }
    }

    pub fn try_move(
        &mut self,
        piece_ref: &Piece,
        end_pos: [u8; 2],
        promotion: Option<&dyn Fn(PieceData) -> Piece>,
    ) -> MoveResult {
        // get the copy in the hashset. We can't be certain that piece_ref references the hashset.
        let piece = self.pieces.remove(&piece_ref.data().position).unwrap();

        let move_type = if piece.data().side != self.turn {
            // if it's not your turn, it doesn't matter, it's already invalid.
            MoveType::Invalid
        } else {
            piece.can_move(self, end_pos, true, promotion)
        };

        self.apply_move(piece, move_type, end_pos, promotion)
    }

    /// This iterates through the entire HashMap to find the king.
    /// There must be a better way
    pub fn king(&self, side: Side) -> Option<&Piece> {
        self.pieces.values().find(|piece|
            match piece {
                Piece::King(data) if data.side == side => true,
                _ => false
            }
        )
    }

    pub fn possible_moves(&self, side: Side) -> Vec<(&Piece, [u8; 2])> {
        let mut moves = Vec::new();

        for piece in self.pieces.values() {
            if piece.data().side != side {
                continue
            }
            for i in 0..8 {
                for j in 0..8 {
                    let move_type = piece.can_move(self, [i, j], true, Some(&Piece::Queen));
                    if move_type != MoveType::Invalid {
                        moves.push((piece, [i, j]));
                    }
                }
            }
        }

        moves
    }

    pub fn is_king_in_check(&self, king: &Piece) -> bool {
        for (_pos, piece) in self.pieces.iter() {
            if piece.data().side == king.data().side {
                continue;
            }
            match piece {
                Piece::Pawn(_data)
                | Piece::Rook(_data)
                | Piece::Knight(_data)
                | Piece::Bishop(_data)
                | Piece::Queen(_data)
                | Piece::King(_data) => {
                    // NOTE: If you must promote, "promote" to a pawn.
                    // This refers to where the pawn directly threatens the
                    // king.
                    // Therefore, what it promotes to doesn't matter.
                    // To emphasize this, we "promote" to a pawn.
                    let move_type =
                        piece.can_move(self, king.data().position, false, Some(&Piece::Pawn));
                    if let MoveType::Capture | MoveType::PawnPromotionCapture = move_type {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn is_side_in_check(&self, side: Side) -> bool {
        let king = self.king(side).unwrap();
        self.is_king_in_check(king)
    }
}
