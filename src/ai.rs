use crate::chessboard::{Checkmate, Chessboard};
use crate::piece::{Piece, Side};
use crate::table::{TranspositionTable, TTEntry, Flag};
use rayon::prelude::*;
use std::fmt;
use std::sync::{Arc, RwLock};

const MAX_SIDE: Side = Side::Light;
const MIN_SIDE: Side = Side::Dark;

// So it seems that using this causes the negamax algorithm to have a double negative
fn side_sign(side: Side) -> i32 {
    match side {
        MAX_SIDE => 1,
        MIN_SIDE => -1,
    }
}

pub fn get_best_move(chessboard: &Chessboard, depth: u8, tt: Arc<RwLock<TranspositionTable>>) -> (&Piece, [u8; 2]) {
    //thread::sleep(time::Duration::new(2, 0));
    let possible_moves = chessboard.possible_moves(chessboard.turn);
    let scores: Vec<_> = possible_moves
        .par_iter()
        .map(|m| {
            let mut temp = chessboard.clone();
            temp.try_move(&m.0, m.1, Some(&Piece::Queen));
            //println!("Considering Move: {}", SimpleMove(*m));
            // using 2 billion to avoid overflow when negating
            -negamax_score(
                &temp,
                depth - 1,
                -2_000_000_000,
                2_000_000_000,
                vec![SimpleMove(*m)],
                Arc::clone(&tt),
            )
        })
        .collect();

    let display: Vec<_> = possible_moves
        .iter()
        .map(|a| SimpleMove(*a))
        .zip(&scores)
        .collect();

    let tt_guard = tt.read().unwrap();

    let mut pv = Vec::new();
    let mut hash = chessboard.zobrist_hash();

    while let Some(entry) = tt_guard.get_for_hash(hash) {
        if let Some(best_move) = &entry.best_move {
            pv.push(best_move);
            hash = Chessboard::update_hash(hash, &best_move);
        } else {
            break
        }
    }

    println!("PV: {:?}", pv);
    drop(tt_guard);

    println!("{:?}", display);

    //if chessboard.turn == MAX_SIDE {
    //println!("{:?}", tt);
    scores
        .iter()
        .zip(possible_moves)
        .max_by(|a, b| a.0.cmp(b.0))
        .unwrap()
        .1
    //} else {
    //scores.iter().zip(possible_moves).min_by(|a, b| a.0.cmp(b.0)).unwrap().1
    //}
}

// The `moves` are just for debugging
fn negamax_score(
    chessboard: &Chessboard,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    moves: Vec<SimpleMove>, // TODO: Get rid of this, we can get the Principle Variation from the TranspositionTable
    tt: Arc<RwLock<TranspositionTable>>
) -> i32 {
    let init_alpha = alpha;

    let tt_guard = tt.read().unwrap();
    let entry = tt_guard.get(&chessboard);
    if let Some(entry) = entry {
        match entry.flag {
            Flag::Exact => {
                println!("TT Hit: {:?}", moves);
                return entry.score
            },
            Flag::Alpha => alpha = i32::max(alpha, entry.score),
            Flag::Beta => beta = i32::min(beta, entry.score),
        }

        if alpha >= beta {
            return entry.score;
        }
    }
    // Do this up here to free the RwLock immediately, instead of holdig onto it while we continue
    // to do work.
    let mut entry = entry.cloned().unwrap_or_else(|| TTEntry::new(&chessboard));
    drop(tt_guard);

    if depth == 0 {
        let score = side_sign(chessboard.turn) * heuristic_score(chessboard);
        return score;
    }
    let mut score = i32::min_value();
    let mut best_move = None;
    let possible_moves = chessboard.possible_moves(chessboard.turn);

    if possible_moves.is_empty() {
        let score = side_sign(chessboard.turn) * heuristic_score(chessboard);
        return score;
    }

    for m in possible_moves.iter() {
        let mut temp = chessboard.clone();
        temp.try_move(&m.0, m.1, Some(&Piece::Queen));
        //println!("{:?}, Considering Move: {}", chessboard.turn, SimpleMove(*m));
        let mut moves = moves.clone();
        moves.push(SimpleMove(*m));

        let new_score = -negamax_score(&temp, depth - 1, -beta, -alpha, moves, Arc::clone(&tt));

        if new_score > score {
            use crate::table::{piece_id, TTMove};
            score = new_score;
            best_move = Some(TTMove {
                piece_id: piece_id(m.0),
                start_pos: m.0.data().position,
                end_pos: m.1,
            });
        }

        alpha = i32::max(alpha, score);
        if alpha >= beta {
            break;
        }
    }

    entry.score = score;
    entry.best_move = best_move;
    if score <= init_alpha {
        entry.flag = Flag::Alpha;
    } else if score >= beta {
        entry.flag = Flag::Beta;
    } else {
        entry.flag = Flag::Exact;
    }
    entry.depth = depth;
    tt.write().unwrap().store(entry);

    score
}

fn heuristic_score(chessboard: &Chessboard) -> i32 {
    // assume stalemate = 0
    // only the side to move (whose turn it is) can be in checkmate
    if chessboard.is_checkmated(chessboard.turn) == Checkmate::Checkmate {
        // if this move checkmates, just return an incredibly high number
        // for the other side (the winning side)
        return 1000 * side_sign(chessboard.turn.other());
    }

    let mut score = 0;
    // only the side to move (chessboard.turn) can be in check
    // because otherwise, they'd have gotton themselves out of check
    if chessboard.is_side_in_check(chessboard.turn) {
        score += 2 * side_sign(chessboard.turn.other());
    }

    for piece in chessboard.pieces().values() {
        score += piece_value(piece) * side_sign(piece.data().side);
    }

    score
}

fn piece_value(piece: &Piece) -> i32 {
    match piece {
        Piece::Pawn(_) => 1,
        Piece::Knight(_) => 3,
        Piece::Bishop(_) => 3,
        Piece::Rook(_) => 5,
        Piece::Queen(_) => 9,
        Piece::King(_) => 0, // both sides should have a king
    }
}

// This is literally only used for debugging the ai
#[derive(Clone)]
pub struct SimpleMove<'a>(pub (&'a Piece, [u8; 2]));

impl<'a> fmt::Display for SimpleMove<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleMove((Piece::King(_), [rank, file])) => {
                write!(f, "K{}{}", (rank + b'a') as char, file + 1)
            }
            SimpleMove((Piece::Queen(_), [rank, file])) => {
                write!(f, "Q{}{}", (rank + b'a') as char, file + 1)
            }
            SimpleMove((Piece::Rook(_), [rank, file])) => {
                write!(f, "R{}{}", (rank + b'a') as char, file + 1)
            }
            SimpleMove((Piece::Bishop(_), [rank, file])) => {
                write!(f, "B{}{}", (rank + b'a') as char, file + 1)
            }
            SimpleMove((Piece::Knight(_), [rank, file])) => {
                write!(f, "N{}{}", (rank + b'a') as char, file + 1)
            }
            SimpleMove((Piece::Pawn(_), [rank, file])) => {
                write!(f, "{}{}", (rank + b'a') as char, file + 1)
            }
        }
    }
}

impl fmt::Debug for SimpleMove<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleMove((Piece::King(_), [rank, file])) => {
                write!(f, "K{}{}", (rank + b'a') as char, file + 1)
            }
            SimpleMove((Piece::Queen(_), [rank, file])) => {
                write!(f, "Q{}{}", (rank + b'a') as char, file + 1)
            }
            SimpleMove((Piece::Rook(_), [rank, file])) => {
                write!(f, "R{}{}", (rank + b'a') as char, file + 1)
            }
            SimpleMove((Piece::Bishop(_), [rank, file])) => {
                write!(f, "B{}{}", (rank + b'a') as char, file + 1)
            }
            SimpleMove((Piece::Knight(_), [rank, file])) => {
                write!(f, "N{}{}", (rank + b'a') as char, file + 1)
            }
            SimpleMove((Piece::Pawn(_), [rank, file])) => {
                write!(f, "{}{}", (rank + b'a') as char, file + 1)
            }
        }
    }
}
