extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::*;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod chessboard;
mod chessboard_controller;
mod chessboard_view;
mod piece;

use crate::chessboard::Chessboard;
use crate::chessboard_controller::ChessboardController;
use crate::chessboard_view::{ChessboardView, ChessboardViewSettings};

pub const BOARD_SIZE: usize = 8;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Chess", [1024, 810])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let chessboard = Chessboard::standard();
    let view_settings = { ChessboardViewSettings::new() };
    let view = ChessboardView::new(view_settings);
    let mut controller = ChessboardController::new(&chessboard);

    // Create a new game and run it.
    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                use graphics::clear;
                clear([0.0; 4], gl);
                view.draw(&controller, &c, gl);
            });
        }
        controller.event(&e);
    }
}
