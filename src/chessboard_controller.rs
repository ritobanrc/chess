use graphics::Image;
use piston::input::GenericEvent;
use drag_controller::{Drag, DragController};
use crate::{Chessboard, ChessboardView};
use crate::piece::Piece;

pub struct PieceRect<'a> {
    pub piece: &'a Piece, 
    pub rect: Rectangle
}

pub struct ChessboardController<'a> {
    pub piece_rects: Vec<PieceRect<'a>>,
    drag_controller: DragController,
    selected: Option<usize>, // This is a terrible hack, but there isn't any other way to have a reference into piece_rects
}

impl<'a> ChessboardController<'a> {
    pub fn new(chessboard: &'a Chessboard, view: &ChessboardView) -> ChessboardController<'a> {
        let mut piece_rects = Vec::new();
        for (_, piece) in chessboard.get_pieces() {
            piece_rects.push(
                PieceRect {
                    piece: piece, 
                    rect: view.get_piece_rect(piece)
                }
                );
        }

        ChessboardController {
            drag_controller: DragController::new(),
            piece_rects: piece_rects,
            selected: None,
        }
    }

    

    pub fn event <E: GenericEvent>(&mut self, e: &E) {
        let drag_controller = &mut self.drag_controller;
        let piece_rects = &mut self.piece_rects;
        let mut selected: Option<usize> = self.selected;
        drag_controller.event(e, |drag| {
            match drag {
                Drag::Interrupt => println!("Interrupt"),
                Drag::Move(x, y) => {
                    //println!("Move {}{}", x, y);
                    if let Some(idx) = selected {
                        piece_rects[idx].rect.update_center(x, y);
                    }
                }
                Drag::Start(x, y) => {
                    //println!("Start {}{}", x, y);
                    for (i, piece_rect) in piece_rects.iter().enumerate() {
                        if piece_rect.rect.is_point_inside(x, y) {
                            println!("Dragging from piece {:?}", piece_rect.piece);
                            selected = Some(i);
                            return true
                        }
                    }
                    return false
                }, 
                Drag::End(_, _) => {
                    selected = None;
                    //println!("End {}{}", x, y);
                }
            }
            true
        });

        if let Some(idx) = selected {
            self.selected = Some(idx);
        }
    }
}


pub struct Rectangle {
    x: f64, 
    y: f64, 
    w: f64, 
    h: f64
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Rectangle {
        Rectangle { x: x, y: y, w: w, h: h }
    }

    pub fn is_point_inside(&self, x: f64, y: f64) -> bool {
        x > self.x && y > self.y && x < self.x + self.w && y < self.y + self.h
    }

    pub fn update_center(&mut self, new_x: f64, new_y: f64) {
        self.x = new_x - self.w/2.0;
        self.y = new_y - self.w/2.0;
    }
}

// TODO: I don't know if this makes any sort of sense
impl Into<Image> for &Rectangle {
    fn into(self) -> Image {
        Image::new().rect([self.x, self.y, self.w, self.h])
    }
}
