use graphics::Image;
use piston::input::GenericEvent;
use drag_controller::{Drag, DragController};
use std::rc::Rc;
use std::collections::HashMap;
use crate::{Chessboard, ChessboardView};
use crate::piece::Piece;

pub struct ChessboardController {
    pub chessboard: Chessboard,
    drag_controller: DragController,
    //piece_rects: HashMap<Rc<Piece>, Rectangle>
}

impl ChessboardController {
    pub fn new(chessboard: Chessboard, view: &ChessboardView) -> ChessboardController {
        //let piece_rects = HashMap::new();
        //for (_, piece) in chessboard.get_pieces() {
            //piece_rects.insert(Rc::new(piece), view.get_piece_rect(&piece));
        //}
        ChessboardController {
            chessboard: chessboard,
            drag_controller: DragController::new(),
            //piece_rects: piece_rects,
        }
    }


    pub fn event <E: GenericEvent>(&mut self, e: &E) {
        self.drag_controller.event(e, |drag| {
            match drag {
                Drag::Interrupt => println!("Interrupt"),
                Drag::Move(x, y) => println!("Move {}{}", x, y), 
                Drag::Start(x, y) => println!("Start {}{}", x, y), 
                Drag::End(x, y) => println!("End {}{}", x, y), 
            }
            true
        })
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
}

// TODO: I don't know if this makes any sort of sense
impl Into<Image> for &Rectangle {
    fn into(self) -> Image {
        Image::new().rect([self.x, self.y, self.w, self.h])
    }
}
