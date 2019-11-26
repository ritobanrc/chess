use crate::chessboard::Chessboard;
use crate::piece::{Side, Piece, PieceData};
use rand::prelude::*;

lazy_static! {
    #[derive(Debug)]
    pub static ref TABLE: [[u64; 12]; 64] = {
        let mut t = [[0; 12]; 64];
        let mut rng = thread_rng();
        for pos in t.iter_mut() {
            for piece in pos.iter_mut() {
                *piece = rng.gen::<u64>();
            }
        }
        t
    };
}


fn piece_id(p: &Piece) -> usize {
    match p {
        Piece::Pawn(PieceData {
            position: _,
            side: Side::Light,
        }) => 0,
        Piece::Rook(PieceData {
            position: _,
            side: Side::Light,
        }) => 1,
        Piece::Knight(PieceData {
            position: _,
            side: Side::Light,
        }) => 2,
        Piece::Bishop(PieceData {
            position: _,
            side: Side::Light,
        }) => 3,
        Piece::Queen(PieceData {
            position: _,
            side: Side::Light,
        }) => 4,
        Piece::King(PieceData {
            position: _,
            side: Side::Light,
        }) => 5,
        Piece::Pawn(PieceData {
            position: _,
            side: Side::Dark,
        }) => 6,
        Piece::Rook(PieceData {
            position: _,
            side: Side::Dark,
        }) => 7,
        Piece::Knight(PieceData {
            position: _,
            side: Side::Dark,
        }) => 8,
        Piece::Bishop(PieceData {
            position: _,
            side: Side::Dark,
        }) => 9,
        Piece::Queen(PieceData {
            position: _,
            side: Side::Dark,
        }) => 10,
        Piece::King(PieceData {
            position: _,
            side: Side::Dark,
        }) => 11,
    }
}

impl Chessboard {
    /// TODO: Actually use Rust's Hasher Trait
    pub fn zobrist_hash(&self) -> u64 {
        let mut result = 0u64;
        for (pos, piece) in self.pieces() {
            result ^= TABLE[(pos[0] * 8 + pos[1]) as usize][piece_id(piece)];
        }
        result
    }

    pub fn update_hash(mut prev: u64, piece: &Piece, pos: [u8; 2]) -> u64 {
        let old_pos = piece.data().position;
        prev ^= TABLE[(old_pos[0] * 8 + old_pos[1]) as usize][piece_id(piece)];
        prev ^= TABLE[(pos[0] * 8 +  pos[1]) as usize][piece_id(piece)];
        prev
    }
}

/// Used to determine the accuracy of the score.
enum Flag {
    /// The score is exact, after having searched all possible moves.
    /// This corresponds to the principle variation (PV)
    Exact,
    /// We know the move is "too good". Beta cutoff was performed (Cut Nodes).
    /// The score returned is a lower bound for the actual score.
    Beta,
    /// No moves score exceeded alpha, also called "fail low" or "all" nodes.
    Alpha,
}

struct Entry {
    hash: u64,
    depth: u8, 
    score: i32,
    flag: Flag,
    age: u8,
}
