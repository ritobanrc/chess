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

/// The number of clusters in the TranspositionTable
const TT_SIZE: usize = 16384;


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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flag {
    /// The score is exact, after having searched all possible moves.
    /// This corresponds to the principle variation (PV)
    Exact,
    /// We know the move is "too good". Beta cutoff was performed (Cut Nodes).
    /// The score returned is a lower bound for the actual score.
    Beta,
    /// No moves score exceeded alpha, also called "fail low" or "all" nodes.
    Alpha,
}

#[derive(Clone, Debug)]
pub struct TTEntry {
    pub hash: u64,
    pub depth: u8, 
    pub score: i32,
    pub flag: Flag,
    // TODO: Stop storing positions as [u8; 2] because that uses a crap ton of unnecessary memory
    // TODO: Refactor everything so you have a "move" type to work with
    // TODO: Figure out what to do with this
    //best_move: [u8; 4],
    pub age: u8,
}

impl TTEntry {
    /// Creates a new TTEntry with all fields 0, and flag Exact, except the hash
    /// This should only be used if the fields are going to be populated immediately afterwards
    pub fn new(chessboard: &Chessboard) -> TTEntry {
        TTEntry {
            hash: chessboard.zobrist_hash(),
            depth: 0,
            score: 0,
            flag: Flag::Exact,
            age: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct Bucket {
    entries: [Option<TTEntry>; 4],
}

impl Bucket {
    pub const fn empty() -> Bucket {
        Bucket {
            entries: [None, None, None, None],
        }
    }
}

pub struct TranspositionTable {
    buckets: Vec<Bucket>,
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        TranspositionTable {
            buckets: vec![Bucket::empty(); TT_SIZE],
        }
    }

    pub fn store(&mut self, entry: TTEntry) {
        let bucket = &mut self.buckets[(entry.hash % TT_SIZE as u64) as usize];

        // If it's already in the buckets, replace it
        for stored_entry_option in &mut bucket.entries {
            if let Some(stored_entry) = stored_entry_option {
                if stored_entry.hash == entry.hash {
                    // replace it
                    *stored_entry_option = Some(entry);
                    return;
                }
            }
        }

        // If we didn't return, that means we need to store in a None location
        for stored_entry_option in &mut bucket.entries {
            if stored_entry_option.is_none() {
                *stored_entry_option = Some(entry);
                return;
            }
        }

        // TODO: Proper error handling, don't panic
        // TODO: Don't iteearate over the bucket twice
        panic!("More than 4 zobrist hash collisions: {:?}", entry);
    }

    pub fn get(&self, chessboard: &Chessboard) -> Option<&TTEntry> {
        // TODO: Incrementally update this hash
        let hash = chessboard.zobrist_hash();
        // I'm not sure if we need to do the modulus in u64
        let bucket = &self.buckets[(hash % TT_SIZE as u64) as usize];

        for entry in &bucket.entries {
            if let Some(entry) = entry {
                if entry.hash == hash {
                    return Some(entry)
                }
            }
        }
        None
    }
}
