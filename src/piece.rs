use crate::chessboard::{Chessboard, MoveResult};

#[derive(PartialEq, Hash, Debug, Clone)]
pub enum Side {
    Light,
    Dark,
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

impl Eq for Piece {}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.get_data() == other.get_data()
    }
}

#[derive(PartialEq)]
enum MoveValidity {
    Invalid, Valid
}

impl Piece {

    #[inline(always)]
    pub fn get_data(&self) -> &PieceData {
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
    pub fn get_data_mut(&mut self) -> &mut PieceData {
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
    fn get_dx_dy(start: [u8; 2], end: [u8; 2]) -> (i8, i8) {
        return ((end[0] as i8) - (start[0] as i8), (end[1] as i8) - (start[1] as i8))
    }

    fn step_through_positions(&self, chessboard: &Chessboard, end_pos: [u8; 2], dx: i8, dy: i8) -> MoveValidity {
        let mut current = [self.get_data().position[0] as i8 + dx.signum(), self.get_data().position[1] as i8 + dy.signum()];
        // while we are on the board
        while let Some(cur) = Chessboard::on_board(current) {
            // if there is another piece of the same color in this spot, invalid
            // this includes end_pos
            if let Some(other_piece) = chessboard.get_piece_at(cur) {
                // if they're on the same side, it's always invalid
                if other_piece.get_data().side == self.get_data().side {
                    return MoveValidity::Invalid
                } else if cur == end_pos {
                    // if they're on opposite sides, it's only valid if this is the end_pos
                    // (i.e capture)
                    return MoveValidity::Valid
                } else {
                    // oppsoite sides, blocking
                    return MoveValidity::Invalid
                }
            }

            // if we got here, the move is valid
            if cur == end_pos {
                return MoveValidity::Valid
            }
            current = [current[0] + dx.signum(), current[1] + dy.signum()];
        }

        panic!("Piece::can_move -- Piece::Bishop -- Iterating over positions failed to arrive at end_pos.")
    }

    fn can_move(&self, chessboard: &Chessboard, end_pos: [u8; 2]) -> MoveValidity {
        match self {
            Piece::Bishop(data) => { 
                let (dx, dy) = Piece::get_dx_dy(data.position, end_pos);
                if dx.abs() != dy.abs() {
                    return MoveValidity::Invalid
                }
                self.step_through_positions(chessboard, end_pos, dx, dy)
            },
            Piece::Rook(data) => { 
                let (dx, dy) = Piece::get_dx_dy(data.position, end_pos);
                if dx != 0 && dy != 0 {
                    return MoveValidity::Invalid
                }
                self.step_through_positions(chessboard, end_pos, dx, dy)
            },
            Piece::Queen(data) => { 
                let (dx, dy) = Piece::get_dx_dy(data.position, end_pos);
                println!("dx: {:?}, dy: {:?}, result: {:?}", dx, dy, dx != 0 && dy != 0);
                if dx.abs() != dy.abs() && dx != 0 && dy != 0 {
                    return MoveValidity::Invalid
                }
                self.step_through_positions(chessboard, end_pos, dx, dy)
            },
            _ => { MoveValidity::Valid }
        }
    }

    pub fn try_move(mut self, chessboard: &mut Chessboard, end_pos: [u8; 2]) -> MoveResult {
        match &self {
            Piece::Pawn(data)
            | Piece::Rook(data)
            | Piece::Knight(data)
            | Piece::Bishop(data)
            | Piece::Queen(data)
            | Piece::King(data) => {
                if self.can_move(&chessboard, end_pos) == MoveValidity::Invalid {
                    chessboard.apply_move(data.position, self);
                    return MoveResult::Invalid
                }
                if let Some(_) = chessboard.get_piece_at(end_pos) {
                    self.get_data_mut().position = end_pos;
                    let captured = chessboard.apply_move(end_pos, self).unwrap();
                    MoveResult::Capture {
                        moved: chessboard.get_piece_at(end_pos).unwrap(),
                        captured: captured
                    }
                } else {
                    self.get_data_mut().position = end_pos;
                    chessboard.apply_move(end_pos, self);
                    MoveResult::Regular(chessboard.get_piece_at(end_pos).unwrap())
                }
            },
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
