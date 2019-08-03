use crate::chessboard::{Chessboard, MoveResult};

#[derive(PartialEq, Hash, Debug, Clone, Copy)]
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
mod pawn_settings {
    use super::Side;
    use crate::BOARD_SIZE;

    #[inline(always)]
    pub fn get_start_rank(side: Side) -> u8 {
        match side {
            Side::Light => 1,
            Side::Dark => BOARD_SIZE - 2,
        }
    }
    #[inline(always)]
    pub fn get_direction(side: Side) -> i8 {
        match side {
            Side::Light => 1,
            Side::Dark => -1,
        }
    }
}

impl Eq for Piece {}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.get_data() == other.get_data()
    }
}

#[derive(PartialEq)]
enum MoveType {
    Invalid,
    Regular,
    Capture,
    Doublestep,
    EnPassant,
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
            self.get_data().position[0] as i8 + dx.signum(),
            self.get_data().position[1] as i8 + dy.signum(),
        ];
        // while we are on the board
        while let Some(cur) = Chessboard::on_board(current) {
            // if there is another piece of the same color in this spot, invalid
            // this includes end_pos
            if let Some(other_piece) = chessboard.get_piece_at(cur) {
                // if they're on the same side, it's always invalid
                if other_piece.get_data().side == self.get_data().side {
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

    fn can_move(&self, chessboard: &Chessboard, end_pos: [u8; 2]) -> MoveType {
        match self {
            Piece::Bishop(data) => {
                let (dx, dy) = Piece::get_dx_dy(data.position, end_pos);
                if dx.abs() != dy.abs() {
                    return MoveType::Invalid;
                }
                self.step_through_positions(chessboard, end_pos, dx, dy)
            }
            Piece::Rook(data) => {
                let (dx, dy) = Piece::get_dx_dy(data.position, end_pos);
                if dx != 0 && dy != 0 {
                    return MoveType::Invalid;
                }
                self.step_through_positions(chessboard, end_pos, dx, dy)
            }
            Piece::Queen(data) => {
                let (dx, dy) = Piece::get_dx_dy(data.position, end_pos);
                //println!("dx: {:?}, dy: {:?}, result: {:?}", dx, dy, dx != 0 && dy != 0);
                if dx.abs() != dy.abs() && dx != 0 && dy != 0 {
                    return MoveType::Invalid;
                }
                self.step_through_positions(chessboard, end_pos, dx, dy)
            }
            Piece::Knight(data) => {
                if let Some(other_piece) = chessboard.get_piece_at(end_pos) {
                    if other_piece.get_data().side == data.side {
                        return MoveType::Invalid;
                    } else {
                        return MoveType::Capture;
                    }
                }

                let (dx, dy) = Piece::get_dx_dy(data.position, end_pos);

                if (dx.abs() == 1 && dy.abs() == 2) || (dx.abs() == 2 && dy.abs() == 1) {
                    MoveType::Regular
                } else {
                    MoveType::Invalid
                }
            }
            Piece::Pawn(data) => {
                let (dx, dy) = Piece::get_dx_dy(data.position, end_pos);

                if dx == 0
                    && dy == pawn_settings::get_direction(data.side)
                    && chessboard.get_piece_at(end_pos) == None
                {
                    // regular move forward
                    MoveType::Regular
                } else if data.position[1] == pawn_settings::get_start_rank(data.side)
                    && dx == 0
                    && dy.abs() == 2
                    && chessboard.get_piece_at([
                        data.position[0],
                        (data.position[1] as i8 + pawn_settings::get_direction(data.side)) as u8,
                    ]) == None
                    && chessboard.get_piece_at(end_pos) == None
                {
                    // doublestep at beginning
                    MoveType::Doublestep
                } else if dx.abs() == 1 && dy == pawn_settings::get_direction(data.side) {
                    // capturing diagonally
                    let capture = chessboard.get_piece_at(end_pos);
                    if let Some(capture) = capture {
                        if capture.get_data().side != data.side {
                            MoveType::Capture
                        } else {
                            MoveType::Invalid
                        }
                    } else {
                        if let Some(en_passant) = chessboard.en_passant {
                            if en_passant[0] == end_pos[0] 
                                && (en_passant[1] as i8 + pawn_settings::get_direction(data.side)) as u8 == end_pos[1] {
                                return MoveType::EnPassant
                            }
                        }
                        MoveType::Invalid
                    }
                } else {
                    MoveType::Invalid
                }
            }
            _ => MoveType::Regular,
        }
    }

    pub fn try_move(mut self, chessboard: &mut Chessboard, end_pos: [u8; 2]) -> MoveResult {
        println!("En Passant Position: {:?}", chessboard.en_passant);

        let move_type = self.can_move(&chessboard, end_pos);
        match move_type {
            MoveType::Invalid => {
                chessboard.apply_move(self.get_data().position, self);
                chessboard.en_passant = None;
                MoveResult::Invalid
            }

            MoveType::Capture => {
                self.get_data_mut().position = end_pos;
                chessboard.en_passant = None;
                let captured = chessboard.apply_move(end_pos, self).unwrap();
                MoveResult::Capture {
                    moved: chessboard.get_piece_at(end_pos).unwrap(),
                    captured,
                }
            }

            MoveType::Regular => {
                self.get_data_mut().position = end_pos;
                chessboard.en_passant = None;
                chessboard.apply_move(end_pos, self);
                MoveResult::Regular(chessboard.get_piece_at(end_pos).unwrap())
            }

            MoveType::Doublestep => {
                self.get_data_mut().position = end_pos;
                chessboard.en_passant = Some(self.get_data().position);
                chessboard.apply_move(end_pos, self);
                MoveResult::Regular(chessboard.get_piece_at(end_pos).unwrap())
            },

            MoveType::EnPassant => { 
                self.get_data_mut().position = end_pos;
                chessboard.apply_move(end_pos, self); // this should not return anything
                let captured = chessboard.pieces.remove(&chessboard.en_passant.unwrap()).unwrap();
                chessboard.en_passant = None;
                MoveResult::EnPassant {
                    moved: chessboard.get_piece_at(end_pos).unwrap(),
                    captured,
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bishop_move_up_left() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Bishop);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [2, 6]);
        assert_eq!(
            chessboard.get_piece_at([2, 6]).unwrap().get_data().position,
            [2, 6]
        );
    }

    #[test]
    fn test_bishop_move_down_left() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Bishop);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [2, 2]);
        assert_eq!(
            chessboard.get_piece_at([2, 2]).unwrap().get_data().position,
            [2, 2]
        );
    }

    #[test]
    fn test_bishop_move_up_right() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Bishop);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [6, 6]);
        assert_eq!(
            chessboard.get_piece_at([6, 6]).unwrap().get_data().position,
            [6, 6]
        );
    }

    #[test]
    fn test_bishop_move_down_right() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Bishop);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [6, 2]);
        assert_eq!(
            chessboard.get_piece_at([6, 2]).unwrap().get_data().position,
            [6, 2]
        );
    }

    #[test]
    fn test_bishop_fails_straight() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Bishop);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        match chessboard.try_move(&piece, [4, 6]) {
            MoveResult::Invalid => {}
            _ => panic!("test_bishop_fails_straight failed."),
        }
    }

    #[test]
    fn test_bishop_fails_irregular() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Bishop);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        match chessboard.try_move(&piece, [6, 5]) {
            MoveResult::Invalid => {}
            _ => panic!("test_bishop_fails_straight failed."),
        }
    }

    #[test]
    fn test_bishop_captures() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Bishop);
        create_piece(&mut chessboard.pieces, [6, 6], Side::Dark, &Piece::Knight);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        let move_result = chessboard.try_move(&piece, [6, 6]);
        if let MoveResult::Capture { moved: _, captured } = move_result {
            match captured {
                Piece::Knight(data) => assert_eq!(data.position, [6, 6]),
                _ => panic!("test_bishop_captures failed."),
            }
            assert_eq!(
                chessboard.get_piece_at([6, 6]).unwrap().get_data().side,
                Side::Light
            );
        } else {
            panic!("test_bishop_captures failed.")
        }
    }

    #[test]
    fn test_bishop_fails_onto_occupied() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Bishop);
        create_piece(&mut chessboard.pieces, [6, 6], Side::Light, &Piece::Knight);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        match chessboard.try_move(&piece, [6, 6]) {
            MoveResult::Invalid => {}
            _ => panic!("test_bishop_fails_onto_occupied failed."),
        }
        if let Piece::Bishop(_data) = chessboard.get_piece_at([4, 4]).unwrap() {
        } else {
            panic!("test_bishop_fails_onto_occupied failed.")
        }
        if let Piece::Knight(_data) = chessboard.get_piece_at([6, 6]).unwrap() {
        } else {
            panic!("test_bishop_fails_onto_occupied failed.")
        }
    }

    #[test]
    fn test_bishop_fails_past_occupied_same() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Bishop);
        create_piece(&mut chessboard.pieces, [6, 6], Side::Light, &Piece::Knight);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        match chessboard.try_move(&piece, [7, 7]) {
            MoveResult::Invalid => {}
            _ => panic!("test_bishop_fails_onto_occupied failed."),
        }
        if let Piece::Bishop(_data) = chessboard.get_piece_at([4, 4]).unwrap() {
        } else {
            panic!("test_bishop_fails_onto_occupied failed.")
        }
        if let Piece::Knight(_data) = chessboard.get_piece_at([6, 6]).unwrap() {
        } else {
            panic!("test_bishop_fails_onto_occupied failed.")
        }
    }

    #[test]
    fn test_bishop_fails_past_occupied_opp() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Bishop);
        create_piece(&mut chessboard.pieces, [6, 6], Side::Dark, &Piece::Knight);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        match chessboard.try_move(&piece, [7, 7]) {
            MoveResult::Invalid => {}
            _ => panic!("test_bishop_fails_onto_occupied failed."),
        }
        if let Piece::Bishop(_data) = chessboard.get_piece_at([4, 4]).unwrap() {
        } else {
            panic!("test_bishop_fails_onto_occupied failed.")
        }
        if let Piece::Knight(_data) = chessboard.get_piece_at([6, 6]).unwrap() {
        } else {
            panic!("test_bishop_fails_onto_occupied failed.")
        }
    }

    #[test]
    fn test_rook_move_up() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Rook);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [4, 6]);
        assert_eq!(
            chessboard.get_piece_at([4, 6]).unwrap().get_data().position,
            [4, 6]
        );
    }

    #[test]
    fn test_rook_move_down() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Rook);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [4, 2]);
        assert_eq!(
            chessboard.get_piece_at([4, 2]).unwrap().get_data().position,
            [4, 2]
        );
    }

    #[test]
    fn test_rook_move_right() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Rook);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [6, 4]);
        assert_eq!(
            chessboard.get_piece_at([6, 4]).unwrap().get_data().position,
            [6, 4]
        );
    }

    #[test]
    fn test_rook_move_left() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Rook);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [2, 4]);
        assert_eq!(
            chessboard.get_piece_at([2, 4]).unwrap().get_data().position,
            [2, 4]
        );
    }

    #[test]
    fn test_rook_fails_diagnol() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Rook);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        match chessboard.try_move(&piece, [6, 6]) {
            MoveResult::Invalid => {}
            _ => panic!("test_rook_fails_straight failed."),
        }
    }

    #[test]
    fn test_rook_fails_irregular() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Rook);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        match chessboard.try_move(&piece, [6, 5]) {
            MoveResult::Invalid => {}
            _ => panic!("test_rook_fails_straight failed."),
        }
    }

    #[test]
    fn test_rook_captures() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Rook);
        create_piece(&mut chessboard.pieces, [4, 6], Side::Dark, &Piece::Knight);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        let move_result = chessboard.try_move(&piece, [4, 6]);
        if let MoveResult::Capture { moved: _, captured } = move_result {
            match captured {
                Piece::Knight(data) => assert_eq!(data.position, [4, 6]),
                _ => panic!("test_rook_captures failed."),
            }
            assert_eq!(
                chessboard.get_piece_at([4, 6]).unwrap().get_data().side,
                Side::Light
            );
        } else {
            panic!("test_rook_captures failed.")
        }
    }

    #[test]
    fn test_rook_fails_onto_occupied() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Rook);
        create_piece(&mut chessboard.pieces, [4, 6], Side::Light, &Piece::Knight);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        match chessboard.try_move(&piece, [4, 6]) {
            MoveResult::Invalid => {}
            _ => panic!("test_rook_fails_onto_occupied failed."),
        }
        if let Piece::Rook(_data) = chessboard.get_piece_at([4, 4]).unwrap() {
        } else {
            panic!("test_rook_fails_onto_occupied failed.")
        }
        if let Piece::Knight(_data) = chessboard.get_piece_at([4, 6]).unwrap() {
        } else {
            panic!("test_rook_fails_onto_occupied failed.")
        }
    }

    #[test]
    fn test_rook_fails_past_occupied_same() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Rook);
        create_piece(&mut chessboard.pieces, [4, 6], Side::Light, &Piece::Knight);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        match chessboard.try_move(&piece, [4, 7]) {
            MoveResult::Invalid => {}
            _ => panic!("test_rook_fails_onto_occupied failed."),
        }
        if let Piece::Rook(_data) = chessboard.get_piece_at([4, 4]).unwrap() {
        } else {
            panic!("test_rook_fails_onto_occupied failed.")
        }
        if let Piece::Knight(_data) = chessboard.get_piece_at([4, 6]).unwrap() {
        } else {
            panic!("test_rook_fails_onto_occupied failed.")
        }
    }

    #[test]
    fn test_rook_fails_past_occupied_opp() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Rook);
        create_piece(&mut chessboard.pieces, [4, 6], Side::Dark, &Piece::Knight);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        match chessboard.try_move(&piece, [4, 7]) {
            MoveResult::Invalid => {}
            _ => panic!("test_rook_fails_onto_occupied failed."),
        }
        if let Piece::Rook(_data) = chessboard.get_piece_at([4, 4]).unwrap() {
        } else {
            panic!("test_rook_fails_onto_occupied failed.")
        }
        if let Piece::Knight(_data) = chessboard.get_piece_at([4, 6]).unwrap() {
        } else {
            panic!("test_rook_fails_onto_occupied failed.")
        }
    }

    #[test]
    fn test_queen_move_up_left() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Queen);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [2, 6]);
        assert_eq!(
            chessboard.get_piece_at([2, 6]).unwrap().get_data().position,
            [2, 6]
        );
    }

    #[test]
    fn test_queen_move_down_left() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Queen);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [2, 2]);
        assert_eq!(
            chessboard.get_piece_at([2, 2]).unwrap().get_data().position,
            [2, 2]
        );
    }

    #[test]
    fn test_queen_move_up_right() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Queen);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [6, 6]);
        assert_eq!(
            chessboard.get_piece_at([6, 6]).unwrap().get_data().position,
            [6, 6]
        );
    }

    #[test]
    fn test_queen_move_down_right() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Queen);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [6, 2]);
        assert_eq!(
            chessboard.get_piece_at([6, 2]).unwrap().get_data().position,
            [6, 2]
        );
    }

    #[test]
    fn test_queen_move_up() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Queen);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [4, 6]);
        assert_eq!(
            chessboard.get_piece_at([4, 6]).unwrap().get_data().position,
            [4, 6]
        );
    }

    #[test]
    fn test_queen_move_down() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Queen);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [4, 2]);
        assert_eq!(
            chessboard.get_piece_at([4, 2]).unwrap().get_data().position,
            [4, 2]
        );
    }

    #[test]
    fn test_queen_move_right() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Queen);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [6, 4]);
        assert_eq!(
            chessboard.get_piece_at([6, 4]).unwrap().get_data().position,
            [6, 4]
        );
    }

    #[test]
    fn test_queen_move_left() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Queen);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [2, 4]);
        assert_eq!(
            chessboard.get_piece_at([2, 4]).unwrap().get_data().position,
            [2, 4]
        );
    }

    #[test]
    fn test_queen_fails_irregular() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Queen);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        match chessboard.try_move(&piece, [6, 5]) {
            MoveResult::Invalid => {}
            _ => panic!("test_rook_fails_straight failed."),
        }
    }

    #[test]
    fn test_knight_all_moves_1() {
        use crate::chessboard::create_piece;
        for dx in [-1, 1].iter() {
            for dy in [-2, 2].iter() {
                let mut chessboard = Chessboard::empty();
                create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Knight);
                let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
                let end_pos = [(4 + dx) as u8, (4 + dy) as u8];
                println!("{:?}", end_pos);
                let move_result = chessboard.try_move(&piece, end_pos);
                println!("{:?}", move_result);
                assert_eq!(
                    chessboard
                        .get_piece_at(end_pos)
                        .unwrap()
                        .get_data()
                        .position,
                    end_pos
                );
            }
        }
    }

    #[test]
    fn test_knight_all_moves_2() {
        use crate::chessboard::create_piece;
        for dx in [-2, 2].iter() {
            for dy in [-1, 1].iter() {
                let mut chessboard = Chessboard::empty();
                create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Knight);
                let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
                let end_pos = [(4 + dx) as u8, (4 + dy) as u8];
                println!("{:?}", end_pos);
                let move_result = chessboard.try_move(&piece, end_pos);
                println!("{:?}", move_result);
                assert_eq!(
                    chessboard
                        .get_piece_at(end_pos)
                        .unwrap()
                        .get_data()
                        .position,
                    end_pos
                );
            }
        }
    }

    #[test]
    fn test_pawn_move_simple_light() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Pawn);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [4, 5]);
        assert_eq!(
            chessboard.get_piece_at([4, 5]).unwrap().get_data().position,
            [4, 5]
        );
    }

    #[test]
    fn test_pawn_move_simple_dark() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Dark, &Piece::Pawn);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        chessboard.try_move(&piece, [4, 3]);
        assert_eq!(
            chessboard.get_piece_at([4, 3]).unwrap().get_data().position,
            [4, 3]
        );
    }

    #[test]
    fn test_pawn_doublestep_light() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 1], Side::Light, &Piece::Pawn);
        let piece = chessboard.get_piece_at([4, 1]).unwrap().clone();
        chessboard.try_move(&piece, [4, 3]);
        assert_eq!(
            chessboard.get_piece_at([4, 3]).unwrap().get_data().position,
            [4, 3]
        );
    }

    #[test]
    fn test_pawn_doublestep_dark() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 6], Side::Dark, &Piece::Pawn);
        let piece = chessboard.get_piece_at([4, 6]).unwrap().clone();
        chessboard.try_move(&piece, [4, 4]);
        assert_eq!(
            chessboard.get_piece_at([4, 4]).unwrap().get_data().position,
            [4, 4]
        );
    }

    #[test]
    fn test_pawn_captures() {
        use crate::chessboard::create_piece;
        let mut chessboard = Chessboard::empty();
        create_piece(&mut chessboard.pieces, [4, 4], Side::Light, &Piece::Pawn);
        create_piece(&mut chessboard.pieces, [5, 5], Side::Dark, &Piece::Knight);
        create_piece(&mut chessboard.pieces, [4, 6], Side::Dark, &Piece::Knight);
        let piece = chessboard.get_piece_at([4, 4]).unwrap().clone();
        let move_result = chessboard.try_move(&piece, [5, 5]);
        if let MoveResult::Capture { moved: _, captured } = move_result {
            match captured {
                Piece::Knight(data) => assert_eq!(data.position, [5, 5]),
                _ => panic!("test_rook_captures failed."),
            }
            assert_eq!(
                chessboard.get_piece_at([5, 5]).unwrap().get_data().side,
                Side::Light
            );
        } else {
            panic!("test_rook_captures failed.")
        }
    }

}
