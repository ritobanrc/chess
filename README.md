# Chess

This is my first major Rust project, where I attempt to implement a chess game and AI

## TODO
- [ ] separate into GUI, Core, and AI crates so they can be used independently. (GUI shouldn't rely on AI)
- [ ] Use [Chess Engine Communication Protocol](https://www.chessprogramming.org/Chess_Engine_Communication_Protocol) to interface between AI and Core. This should allow us to test out stockfish as well. 
- [ ] Implement repition draws, 50-move-rule
- [ ] Debug Transposition Table
- [ ] Implement Killer Move Heuristic
- [ ] Document everything
- [ ] Stop using a HashMap to store piece (Consider BitBoards or smth)
- [ ] Stop storing positions as [u8; 2]. There's 64 possibilities, there's no reason to spend 16 bytes. An enum would work well here.
- [ ] Make a consistent `Move` API. Right now, we have a mess of `(&'a Piece, [u8; 2])`, those two as separate arguments to a function, `SimpleMove` struct, `MoveType`s, `MoveResult`s
- [ ] Rearrange `Piece` struct so that `data` calls are not actually function calls (i.e. `Piece` is a struct w/ an enum `PieceType`, `Position`, and `Side`)
- [ ] Make `Side`s and square colors consistent. Right now, we use `Light` and `White`, or `Dark` and `Black` interchangably
- [ ] Optimize `Chessboard::possible_moves`
- [ ] Optimize `Chessboard::can_move` (i.e. don't clone the entire board)
- [ ] Implement opening book and endgame tables
- [ ] Evaluation: Pawn Structure
- [ ] Evaluation: Mobility
- [ ] Evaluation: Center Control/Piece Square Tables
- [ ] Evaluation: King Safety
- [ ] Optimize: Incrementally updated attack tables?



