pub struct ChessboardController {
    chessboard: Chessboard,
}

impl ChessboardController {
    pub fn new(chessboard: Chessboard) -> ChessboardController {
        ChessboardController {
            chessboard: chessboard,
        }
    }


    pub fn event <E: GenericEvent>(&mut self, e: &E) {
    }
}

