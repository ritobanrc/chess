use crate::chessboard::{CastleRights, Chessboard};
use crate::BOARD_SIZE;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Side {
    Light,
    Dark,
}

impl Side {
    #[inline(always)]
    pub fn back_rank(self) -> u8 {
        match self {
            Side::Light => 0,
            Side::Dark => BOARD_SIZE - 1,
        }
    }

    pub fn other(self) -> Side {
        match self {
            Side::Light => Side::Dark,
            Side::Dark => Side::Light,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
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
mod pawn_settings {
    use super::*;

    #[inline(always)]
    pub fn start_rank(side: Side) -> u8 {
        match side {
            Side::Light => 1,
            Side::Dark => BOARD_SIZE - 2,
        }
    }

    #[inline(always)]
    pub fn direction(side: Side) -> i8 {
        match side {
            Side::Light => 1,
            Side::Dark => -1,
        }
    }
}

impl Eq for Piece {}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.data() == other.data()
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum MoveType {
    Invalid,
    Regular,
    Capture,
    Doublestep,
    EnPassant,
    Castle,
    PawnPromotion,
    PawnPromotionCapture,
}

impl Piece {
    #[inline(always)]
    pub fn data(&self) -> &PieceData {
        match &self {
            Piece::Pawn(data)
            | Piece::Rook(data)
            | Piece::Knight(data)
            | Piece::Bishop(data)
            | Piece::Queen(data)
            | Piece::King(data) => &data,
        }
    }

    #[inline(always)]
    pub fn data_mut(&mut self) -> &mut PieceData {
        match self {
            Piece::Pawn(data)
            | Piece::Rook(data)
            | Piece::Knight(data)
            | Piece::Bishop(data)
            | Piece::Queen(data)
            | Piece::King(data) => data,
        }
    }

    #[inline(always)]
    pub fn dx_dy(start: [u8; 2], end: [u8; 2]) -> (i8, i8) {
        (
            (end[0] as i8) - (start[0] as i8),
            (end[1] as i8) - (start[1] as i8),
        )
    }

    fn step_through_positions(
        &self,
        chessboard: &Chessboard,
        end_pos: [u8; 2],
        dx: i8,
        dy: i8,
    ) -> MoveType {
        let mut current = [
            self.data().position[0] as i8 + dx.signum(),
            self.data().position[1] as i8 + dy.signum(),
        ];
        // while we are on the board
        while let Some(cur) = Chessboard::on_board(current) {
            // if there is another piece of the same color in this spot, invalid
            // this includes end_pos
            if let Some(other_piece) = chessboard.piece_at(cur) {
                // if they're on the same side, it's always invalid
                if other_piece.data().side == self.data().side {
                    return MoveType::Invalid;
                } else if cur == end_pos {
                    // if they're on opposite sides, it's only valid if this is the end_pos
                    // (i.e capture)
                    return MoveType::Capture;
                } else {
                    // oppsoite sides, blocking
                    return MoveType::Invalid;
                }
            }
            // if we got here, the move is valid
            if cur == end_pos {
                return MoveType::Regular;
            }
            current = [current[0] + dx.signum(), current[1] + dy.signum()];
        }
        panic!("Piece::step_through_positions -- Iterating over positions failed to arrive at end_pos.")
    }

    pub fn can_move(
        &self,
        chessboard: &Chessboard,
        end_pos: [u8; 2],
        check_check: bool,
        promotion: Option<&dyn Fn(PieceData) -> Piece>,
    ) -> MoveType {
        let original_move_type = match self {
            Piece::Bishop(data) => {
                let (dx, dy) = Piece::dx_dy(data.position, end_pos);
                if dx.abs() != dy.abs() {
                    return MoveType::Invalid;
                }
                self.step_through_positions(chessboard, end_pos, dx, dy)
            }
            Piece::Rook(data) => {
                let (dx, dy) = Piece::dx_dy(data.position, end_pos);
                if dx != 0 && dy != 0 {
                    return MoveType::Invalid;
                }
                self.step_through_positions(chessboard, end_pos, dx, dy)
            }
            Piece::Queen(data) => {
                let (dx, dy) = Piece::dx_dy(data.position, end_pos);
                //println!("dx: {:?}, dy: {:?}, result: {:?}", dx, dy, dx != 0 && dy != 0);
                if dx.abs() != dy.abs() && dx != 0 && dy != 0 {
                    return MoveType::Invalid;
                }
                self.step_through_positions(chessboard, end_pos, dx, dy)
            }
            Piece::Knight(data) => {
                let (dx, dy) = Piece::dx_dy(data.position, end_pos);
                if (dx.abs() == 1 && dy.abs() == 2) || (dx.abs() == 2 && dy.abs() == 1) {
                    if let Some(other_piece) = chessboard.piece_at(end_pos) {
                        if other_piece.data().side == data.side {
                            MoveType::Invalid
                        } else {
                            MoveType::Capture
                        }
                    } else {
                        MoveType::Regular
                    }
                } else {
                    MoveType::Invalid
                }
            }
            Piece::Pawn(data) => {
                let (dx, dy) = Piece::dx_dy(data.position, end_pos);
                if dx == 0
                    && dy == pawn_settings::direction(data.side)
                    && chessboard.piece_at(end_pos) == None
                {
                    // regular move forward
                    if end_pos[1] == data.side.other().back_rank() {
                        MoveType::PawnPromotion
                    } else {
                        MoveType::Regular
                    }
                } else if data.position[1] == pawn_settings::start_rank(data.side)
                    && dx == 0
                    && dy.abs() == 2
                    && chessboard.piece_at([
                        data.position[0],
                        (data.position[1] as i8 + pawn_settings::direction(data.side)) as u8,
                    ]) == None
                    && chessboard.piece_at(end_pos) == None
                {
                    // doublestep at beginning
                    MoveType::Doublestep
                } else if dx.abs() == 1 && dy == pawn_settings::direction(data.side) {
                    // capturing diagonally
                    let capture = chessboard.piece_at(end_pos);
                    if let Some(capture) = capture {
                        if capture.data().side != data.side {
                            //MoveType::Capture
                            if end_pos[1] == data.side.other().back_rank() {
                                MoveType::PawnPromotionCapture
                            } else {
                                MoveType::Capture
                            }
                        } else {
                            MoveType::Invalid
                        }
                    } else if let Some(en_passant) = chessboard.en_passant {
                        if en_passant[0] == end_pos[0]
                            && (en_passant[1] as i8 + pawn_settings::direction(data.side)) as u8
                                == end_pos[1]
                        {
                            MoveType::EnPassant
                        } else {
                            MoveType::Invalid
                        }
                    } else {
                        MoveType::Invalid
                    }
                } else {
                    MoveType::Invalid
                }
            }
            Piece::King(data) => {
                let (dx, dy) = Piece::dx_dy(data.position, end_pos);
                if dx.abs() <= 1 && dy.abs() <= 1 {
                    if let Some(other_piece) = chessboard.piece_at(end_pos) {
                        if other_piece.data().side == data.side {
                            return MoveType::Invalid;
                        }
                        MoveType::Capture
                    } else {
                        MoveType::Regular
                    }
                } else {
                    // castling
                    // Check for castling. From wikipedia, the following
                    // conditions are necessary.
                    //The king and the chosen rook are on the player's first rank.[3]
                    // Neither the king nor the chosen rook has previously moved.
                    //There are no pieces between the king and the chosen rook.
                    //The king is not currently in check.
                    //The king does not pass through a square that is attacked by an enemy piece.[4] TODO
                    //The king does not end up in check. (True of any legal move.)

                    // 1 and 2 automatically checked here
                    let castle_type = chessboard
                        .castle_rights(data.side)
                        .check_end_pos(end_pos, data.side);
                    if castle_type == CastleRights::NoRights {
                        return MoveType::Invalid;
                    }

                    let rook = castle_type.rook_init_pos(data.side).unwrap();
                    //There are no pieces between the king and the chosen rook.
                    let iter = if castle_type == CastleRights::KingSide {
                        data.position[0] + 1..rook[0]
                    } else {
                        rook[0] + 1..data.position[0]
                    };

                    for x in iter {
                        if let Some(_p) = chessboard.piece_at([x, data.side.back_rank()]) {
                            return MoveType::Invalid;
                        }
                    }

                    // The king is not currently in check.
                    // We can't just use the currently existing copy of the board, because the king
                    // has been removed from there.
                    {
                        // instead, create a temporary copy of the chessboard, insert the king there, and check.
                        // While it may be possible to move a clone into and out of the actual
                        // chessboard, I'm not certain if other stuff will break. Better to make a
                        // clone.
                        let mut temp_chessboard = chessboard.clone();
                        temp_chessboard.insert(data.position, self.clone());
                        if temp_chessboard.is_king_in_check(&self) {
                            return MoveType::Invalid;
                        }
                    }
                    {
                        // check if the king will move through check.
                        let mut temp_chessboard = chessboard.clone();
                        let (dx, _dy) = Piece::dx_dy(data.position, end_pos);
                        let mut temp_king = self.clone();
                        let temp_pos = [
                            (data.position[0] as i8 + dx.signum()) as u8,
                            data.position[1],
                        ];
                        temp_king.data_mut().position = temp_pos;
                        temp_chessboard.insert(temp_pos, temp_king);
                        if temp_chessboard
                            .is_king_in_check(temp_chessboard.piece_at(temp_pos).unwrap())
                        {
                            return MoveType::Invalid;
                        }
                    }

                    MoveType::Castle
                }
            }
        };

        if check_check && original_move_type != MoveType::Invalid {
            // clone the chessboard, pretend to apply this move, and check if the king
            // is in check
            let mut temp_board = chessboard.clone();
            temp_board.apply_move(self.clone(), original_move_type, end_pos, promotion);
            //println!("{:?} is king in temp_board after {:?} to {:?}", temp_board.get_king(self.get_data().side), self, end_pos);
            if temp_board.is_side_in_check(self.data().side) {
                //println!("Check for check failed");
                MoveType::Invalid
            } else {
                original_move_type
            }
        } else {
            original_move_type
        }
    }
}

#[derive(Debug, Clone)]
pub enum Piece {
    Pawn(PieceData),
    Rook(PieceData),
    Knight(PieceData),
    Bishop(PieceData),
    Queen(PieceData),
    King(PieceData),
}
