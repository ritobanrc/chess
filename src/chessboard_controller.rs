use crate::ai;
use crate::chessboard::{Checkmate, Chessboard, MoveResult};
use crate::piece::{Piece, PieceData, Side};
use crate::table::TranspositionTable;
use crate::sidebar::Sidebar;
use crate::{BOARD_BORDER_SIZE, BOARD_SIZE, HEIGHT};
use drag_controller::{Drag, DragController};
use graphics::Image;
use piston::input::GenericEvent;
use std::fmt::Write;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;

static AI_LEVEL: u8 = 3;
static AI_SIDE: Side = Side::Dark;
static AI: bool = true;

pub struct PieceRect {
    pub piece: Piece,
    pub rect: Rectangle,
}

pub struct CaptureCount {
    queen_count: u8,
    rook_count: u8,
    bishop_count: u8,
    knight_count: u8,
    pawn_count: u8,
}

impl CaptureCount {
    pub fn new() -> CaptureCount {
        CaptureCount {
            queen_count: 0,
            rook_count: 0,
            bishop_count: 0,
            knight_count: 0,
            pawn_count: 0,
        }
    }

    pub fn add_piece(&mut self, piece: &Piece) {
        match piece {
            Piece::Queen(_) => {
                self.queen_count += 1;
            }
            Piece::Rook(_) => {
                self.rook_count += 1;
            }
            Piece::Bishop(_) => {
                self.bishop_count += 1;
            }
            Piece::Knight(_) => {
                self.knight_count += 1;
            }
            Piece::Pawn(_) => {
                self.pawn_count += 1;
            }
            Piece::King(_) => panic!("We captured a king!"),
        }
    }

    pub fn display(&self) -> String {
        let mut result = String::new();
        if self.queen_count > 0 {
            write!(&mut result, "{} ♛  ", self.queen_count).unwrap();
        }
        if self.rook_count > 0 {
            write!(&mut result, "{} ♜  ", self.rook_count).unwrap();
        }
        if self.bishop_count > 0 {
            write!(&mut result, "{} ♝  ", self.bishop_count).unwrap();
        }
        if self.knight_count > 0 {
            write!(&mut result, "{} ♞  ", self.knight_count).unwrap();
        }
        if self.pawn_count > 0 {
            write!(&mut result, "{} ♟  ", self.pawn_count).unwrap();
        }

        result
    }
}

pub struct ChessboardController {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    pub piece_rects: Vec<PieceRect>,
    drag_controller: DragController,
    // This is a terrible hack,
    // but there isn't any other way to have a reference into piece_rects,
    // and I can't be bothered to refactor everything
    selected: Option<usize>,
    // currently, this is only used in pawn promotion,
    // the move isn't triggered immediately after the drag stops
    pawn_promotion_move: Option<[u8; 2]>,
    light_capture: CaptureCount,
    dark_capture: CaptureCount,
    // logically, it's not possible for both of these to be true at the same time
    light_check: bool,
    dark_check: bool,
    pub game_result: (Checkmate, Side),
    ai_rx: Option<mpsc::Receiver<(&'static Piece, [u8; 2])>>,
    tt: Option<Arc<RwLock<TranspositionTable>>>,
    chessboard: Chessboard,
}

impl ChessboardController {
    pub fn new(chessboard: Chessboard) -> ChessboardController {
        let piece_rects = Vec::new();
        ChessboardController {
            position: [BOARD_BORDER_SIZE; 2],
            size: HEIGHT - 2.0 * BOARD_BORDER_SIZE,
            piece_rects,
            drag_controller: DragController::new(),
            selected: None,
            pawn_promotion_move: None,
            light_capture: CaptureCount::new(),
            dark_capture: CaptureCount::new(),
            light_check: false,
            dark_check: false,
            // while I could go to the effort of making this an Option<Side>, I don't think it's
            // worth it
            game_result: (Checkmate::Nothing, Side::Light),
            ai_rx: None,
            tt: match AI { 
                true => Some(Arc::new(RwLock::new(TranspositionTable::new()))),
                false => None,
            },
            chessboard,
        }
    }

    pub fn init_piece_rects(&mut self) {
        for piece in self.chessboard.pieces().values() {
            self.piece_rects.push(PieceRect {
                piece: piece.clone(),
                rect: self.piece_rect(piece),
                //rect: Rectangle::new(1.0, 1.0, 1.0, 1.0)
            });
        }
    }

    #[inline(always)]
    pub fn piece_rect(&self, piece: &Piece) -> Rectangle {
        self.square_rect(piece.data().position)
    }

    pub fn square_rect(&self, pos: [u8; 2]) -> Rectangle {
        let square_size = self.square_size();
        Rectangle::new(
            self.position[0] + f64::from(pos[0]) * square_size,
            (self.position[1] + self.size - square_size) - (f64::from(pos[1]) * square_size),
            square_size,
            square_size,
        )
    }

    pub fn turn(&self) -> Side {
        self.chessboard.turn
    }

    #[inline(always)]
    pub fn square_size(&self) -> f64 {
        self.size / f64::from(BOARD_SIZE)
    }

    #[inline(always)]
    pub fn captures(&self, side: Side) -> &CaptureCount {
        match side {
            Side::Light => &self.light_capture,
            Side::Dark => &self.dark_capture,
        }
    }

    #[inline(always)]
    pub fn check(&self, side: Side) -> bool {
        match side {
            Side::Light => self.light_check,
            Side::Dark => self.dark_check,
        }
    }

    #[inline(always)]
    pub fn captures_mut(&mut self, side: Side) -> &mut CaptureCount {
        match side {
            Side::Light => &mut self.light_capture,
            Side::Dark => &mut self.dark_capture,
        }
    }

    fn try_move(
        &mut self,
        idx: usize,
        pos: [u8; 2],
        promotion: Option<&dyn Fn(PieceData) -> Piece>,
    ) {
        let side = self.piece_rects[idx].piece.data().side;
        // get the chessboard, tell it to try the move.
        let piece  = &self.piece_rects[idx].piece;


        let move_result = self
            .chessboard
            .try_move(piece, pos, promotion);


        // In the event that some idiot (cough..me..cough) made it so the
        // chessboard pieces aren't directly linked to the piece rect pieces,
        // set the piece_rect pieces to be the chessboard pieces
        match move_result {
            MoveResult::Invalid => {
                // it it's invalid, set the position equal to wherever it is.
                self.piece_rects[idx].rect =
                    self.square_rect(self.piece_rects[idx].piece.data().position);
            }
            MoveResult::Regular(p) | MoveResult::PawnPromotion(p) => {
                // it it's a regular position, update both the rect and the piece
                self.piece_rects[idx].piece = p.clone();
                self.piece_rects[idx].rect =
                    self.square_rect(self.piece_rects[idx].piece.data().position);
            }
            MoveResult::Capture { moved, captured }
            | MoveResult::EnPassant { moved, captured }
            | MoveResult::PawnPromotionCapture { moved, captured } => {
                // start by updating the moved piece
                self.piece_rects[idx].piece = moved.clone();
                self.piece_rects[idx].rect =
                    self.square_rect(self.piece_rects[idx].piece.data().position);
                // add it to captured list
                self.captures_mut(captured.data().side).add_piece(&captured);
                // now remove the captured piece
                let pos = self
                    .piece_rects
                    .iter()
                    .position(|x| x.piece == captured)
                    .unwrap();
                self.piece_rects.remove(pos);
            }
            MoveResult::Castle {
                king,
                rook,
                rook_init_pos,
            } => {
                // it it's a regular position, update both the rect and the piece
                self.piece_rects[idx].piece = king.clone();
                let rook = rook.clone();
                self.piece_rects[idx].rect =
                    self.square_rect(self.piece_rects[idx].piece.data().position);
                let pos = self
                    .piece_rects
                    .iter()
                    .position(|x| x.piece.data().position == rook_init_pos)
                    .unwrap();
                self.piece_rects[pos].piece = rook;
                self.piece_rects[pos].rect =
                    self.square_rect(self.piece_rects[pos].piece.data().position);
            }
        };

        self.game_result = (self.chessboard.is_checkmated(side.other()), side);
        match self.game_result.0 {
            Checkmate::Checkmate => {
                println!("Game Over! {:?} wins", self.game_result.1);
                return;
            }
            Checkmate::Stalemate => {
                println!("Game Over! Draw");
                return;
            }
            _ => {}
        }

        self.light_check = self.chessboard.is_side_in_check(Side::Light);
        self.dark_check = self.chessboard.is_side_in_check(Side::Dark);
        if self.light_check {
            println!("White in Check");
        }
        if self.dark_check {
            println!("Black in Check");
        }


        //println!("{:?}", self.chessboard.zobrist_hash());

        if AI && self.chessboard.turn == AI_SIDE {
            let (tx, rx) = mpsc::channel();

            struct ChessboardPtr(*const Chessboard);

            unsafe impl Send for ChessboardPtr {}
            unsafe impl Sync for ChessboardPtr {}

            // This needs to be unsafe because I'm sending an immutable reference to the ai
            // While the AI runs, event (which takes a controller, and thus mutable chessboard) still needs to run
            // There is no way to convince the borrow checker than `event` won't actually modify
            // chessboard (because it's not the other person's turn). That is a logical distinction that I can make based on how turns in
            // chess work.
            let chessboard = ChessboardPtr(&self.chessboard as *const Chessboard);

            let tt = Arc::clone(&self.tt.as_ref().unwrap());

            //let best_move = rx.recv().unwrap();
            thread::spawn(move || {
                let chessboard = unsafe { &(*chessboard.0) };
                let best_move = ai::get_best_move(chessboard, AI_LEVEL, tt);
                use ai::SimpleMove;
                println!("Found best Move: {:?}", SimpleMove(best_move));
                tx.send(best_move).unwrap();
            });

            self.ai_rx = Some(rx);
        }
    }

    fn piece_idx_in_piece_rects(&self, piece: &Piece) -> usize {
        self.piece_rects
            .iter()
            .position(|piece_rect| &piece_rect.piece == piece)
            .unwrap()
    }

    /// Handle events to the chessboard (piece dragging)
    pub fn event<E: GenericEvent>(&mut self, e: &E, sidebar: &mut Sidebar) {
        // check if the AI has sent back something
        // I can't seem to figure out why UpdateEvent isn't called
        // So there is a slight lag between when the best move is found
        // And when the move is actually made
        if e.update_args().is_some() {
            if let Some(rx) = &self.ai_rx {
                match rx.try_recv() {
                    Ok(best_move) => {
                        let piece = self.piece_idx_in_piece_rects(best_move.0);
                        let pos = best_move.1;
                        self.try_move(piece, pos, Some(&Piece::Queen));
                        self.ai_rx = None;
                    }
                    Err(mpsc::TryRecvError::Empty) => { /* nothing's happened, keep going */ }
                    Err(mpsc::TryRecvError::Disconnected) => panic!("AI disconnected"),
                }
                return;
            }
        }

        // if there is an AI currently running, the player cannot interact with the pieces.
        // otherwise, the unsafe used to a get an immutable reference to the chessboard is
        // actually unsafe. This has to be separate, because it applies to all events, not just
        // update events.
        if self.ai_rx.is_some() {
            return;
        }


        let drag_controller = &mut self.drag_controller;
        let piece_rects = &self.piece_rects;
        let turn = &self.chessboard.turn;
        let mut selected: Option<usize> = self.selected;

        let mut local_drag: Option<Drag> = None;

        drag_controller.event(e, |drag| {
            // start is the only case we need to handle in the closure.
            if let Drag::Start(x, y) = drag {
                //println!("Start {}{}", x, y);
                for (i, piece_rect) in piece_rects.iter().enumerate() {
                    if piece_rect.piece.data().side == *turn
                        && piece_rect.rect.is_point_inside(x, y)
                    {
                        selected = Some(i);
                        return true;
                    }
                }
                return false;
            } else {
                // for the rest, cache the result in the local, and handle it outside
                local_drag = Some(drag)
            }
            true
        });

        self.selected = selected;
        if let Some(drag) = local_drag {
            match drag {
                Drag::Interrupt | Drag::Start(_, _) => {} // do nothing. start already handled
                Drag::Move(x, y) => {
                    //println!("Move {}{}", x, y);
                    if let Some(idx) = self.selected {
                        self.piece_rects[idx].rect.update_center(x, y);
                    }
                }
                Drag::End(x, y) => {
                    // if something is selected
                    if let Some(idx) = selected {
                        // this feels about right.
                        let pos: [u8; 2] = [
                            ((x - self.position[0]) / self.square_size()).floor() as u8,
                            BOARD_SIZE - ((y - self.position[0]) / self.square_size()).ceil() as u8,
                        ];

                        let piece = &self.piece_rects[idx].piece;
                        if pos == piece.data().position {
                            self.piece_rects[idx].rect =
                                self.square_rect(self.piece_rects[idx].piece.data().position);
                            return;
                        }

                        match piece {
                            Piece::Pawn(_data)
                                if pos[1] == piece.data().side.other().back_rank() =>
                            {
                                sidebar.add_pawn_buttons();
                                self.pawn_promotion_move = Some(pos);
                            }
                            _ => {
                                self.try_move(idx, pos, None);
                                self.selected = None; // drag over, no longer selected
                            }
                        }
                    }
                }
            };
        }
    }

    pub fn trigger_pawn_promotion(&mut self, promotion: &dyn Fn(PieceData) -> Piece) {
        self.try_move(
            self.selected.unwrap(),
            self.pawn_promotion_move.unwrap(),
            Some(promotion),
        );
        self.selected = None;
    }
}

// NOTE: There's no reason for this to be in chessboard_controller, but I'm too lazy to move it.
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Rectangle {
        Rectangle { x, y, w, h }
    }

    pub fn is_point_inside(&self, x: f64, y: f64) -> bool {
        x > self.x && y > self.y && x < self.x + self.w && y < self.y + self.h
    }

    pub fn offset(mut self, offset: [f64; 2]) -> Rectangle {
        self.x += offset[0];
        self.y += offset[1];
        self
    }

    pub fn update_center(&mut self, new_x: f64, new_y: f64) {
        self.x = new_x - self.w / 2.0;
        self.y = new_y - self.w / 2.0;
    }

    pub fn left(&self) -> f64 {
        self.x
    }

    pub fn top(&self) -> f64 {
        self.y
    }

    pub fn size_x(&self) -> f64 {
        self.w
    }

    pub fn size_y(&self) -> f64 {
        self.h
    }

    pub fn right(&self) -> f64 {
        self.x + self.w
    }

    pub fn bottom(&self) -> f64 {
        self.y + self.h
    }

    pub fn center_x(&self) -> f64 {
        self.x + self.w / 2.0
    }

    pub fn center_y(&self) -> f64 {
        self.y + self.h / 2.0
    }

    pub fn center(&self) -> [f64; 2] {
        [self.center_x(), self.center_y()]
    }
}

impl From<[f64; 4]> for Rectangle {
    #[inline(always)]
    fn from(v: [f64; 4]) -> Self {
        Rectangle::new(v[0], v[1], v[2], v[3])
    }
}

impl Into<[f64; 4]> for Rectangle {
    #[inline(always)]
    fn into(self) -> [f64; 4] {
        [self.x, self.y, self.w, self.h]
    }
}

// TODO: I don't know if this makes any sort of sense
// I also don't know why we're using references for a Copy type. Maybe past me knows.
impl Into<Image> for &Rectangle {
    #[inline(always)]
    fn into(self) -> Image {
        Image::new().rect([self.x, self.y, self.w, self.h])
    }
}
