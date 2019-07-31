use crate::piece::Piece;
use crate::chessboard::{MoveResult, Chessboard};
use crate::BOARD_SIZE;
use drag_controller::{Drag, DragController};
use graphics::Image;
use piston::input::GenericEvent;
use std::iter;

pub struct PieceRect {
    pub piece: Piece,
    pub rect: Rectangle,
}

pub struct ChessboardController {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    pub piece_rects: Vec<PieceRect>,
    drag_controller: DragController,
    selected: Option<usize>, // This is a terrible hack, but there isn't any other way to have a reference into piece_rects
    chessboard: Chessboard
}

impl ChessboardController {
    pub fn new(chessboard: Chessboard) -> ChessboardController {
        let piece_rects = Vec::new();
        let controller = ChessboardController {
            position: [5.0; 2],
            size: 800.0,
            piece_rects,
            drag_controller: DragController::new(),
            selected: None,
            chessboard
        };
        controller
    }

    pub fn init_piece_rects(&mut self) {
        for piece in self.chessboard.get_pieces().values() {
            self.piece_rects.push(PieceRect {
                piece: piece.clone(),
                rect: self.get_piece_rect(piece),
                //rect: Rectangle::new(1.0, 1.0, 1.0, 1.0)
            });
        }
    }


    #[inline(always)]
    pub fn get_piece_rect(&self, piece: &Piece) -> Rectangle {
        self.get_square_rect(piece.get_data().position)
    }

    pub fn get_square_rect(&self, pos: [u8; 2]) -> Rectangle {
        let square_size = self.square_size();
        Rectangle::new(
            self.position[0] + f64::from(pos[0]) * square_size,
            (self.position[1] + self.size - square_size)
                - (f64::from(pos[1]) * square_size),
            square_size,
            square_size,
        )
    }

    #[inline(always)]
    pub fn square_size(&self) -> f64 {
        self.size / (BOARD_SIZE as f64)
    }


    /// Handle events to the chessboard (piece dragging)
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        let drag_controller = &mut self.drag_controller;
        let piece_rects = &self.piece_rects;
        let mut selected: Option<usize> = self.selected;

        let mut local_drag: Option<Drag> = None;

        drag_controller.event(e, |drag| {
            // start is the only case we need to handle in the closure. 
            if let Drag::Start(x, y) = drag {
                    //println!("Start {}{}", x, y);
                    for (i, piece_rect) in piece_rects.iter().enumerate() {
                        if piece_rect.rect.is_point_inside(x, y) {
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

        if let Some(drag) = local_drag {
            match drag {
                Drag::Interrupt | Drag::Start(_, _) => { } // do nothing. start already handled
                Drag::Move(x, y) => {
                    //println!("Move {}{}", x, y);
                    if let Some(idx) = selected  {
                        self.piece_rects[idx].rect.update_center(x, y);
                    }
                }
                Drag::End(x, y) => {
                    // this feels about right.
                    if let Some(idx) = selected {
                        let pos: [u8; 2] = [((x - self.position[0])/self.square_size()).floor() as u8,
                                            BOARD_SIZE - ((y - self.position[0])/self.square_size()).ceil() as u8];

                        // get the chessboard, tell it to try the move. 
                        let move_result = self.chessboard.try_move(&self.piece_rects[idx].piece, pos);

                        // In the event that some idiot (cough..me..cough) made it so the
                        // chessboard pieces aren't directly linked to the piece rect pieces, 
                        // set the piece_rect pieces to be the chessboard pieces
                        match move_result {
                            MoveResult::Invalid => {
                                // it it's invalid, set the position equal to wherever it is.
                                self.piece_rects[idx].rect = self.get_square_rect(self.piece_rects[idx].piece.get_data().position);
                            },
                            MoveResult::Regular(p) => {
                                // it it's a regular position, update both the rect and the piece
                                self.piece_rects[idx].piece = p.clone();
                                self.piece_rects[idx].rect = self.get_square_rect(self.piece_rects[idx].piece.get_data().position);
                            },
                            MoveResult::Capture{moved, captured} => {
                                // start by updating the moved piece
                                self.piece_rects[idx].piece = moved.clone();
                                self.piece_rects[idx].rect = self.get_square_rect(self.piece_rects[idx].piece.get_data().position);
                                // now remove the captured piece
                                let pos = self.piece_rects.iter().position(|x| x.piece == captured).unwrap();
                                self.piece_rects.remove(pos);
                            }
                            _ => { }
                        };
                        selected = None; // drag over, no longer selected
                    }
                }
            };
        }
        self.selected = selected;
    }
}

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

    pub fn update_center(&mut self, new_x: f64, new_y: f64) {
        self.x = new_x - self.w / 2.0;
        self.y = new_y - self.w / 2.0;
    }
}

// TODO: I don't know if this makes any sort of sense
impl Into<Image> for &Rectangle {
    #[inline(always)]
    fn into(self) -> Image {
        Image::new().rect([self.x, self.y, self.w, self.h])
    }
}
