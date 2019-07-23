extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod chessboard;
mod piece;

pub use crate::chessboard::{Chessboard, ChessboardController, ChessboardView};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            use graphics::{clear};

            clear([0.0; 4], gl);
        });
    }

}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Chess",
            [512, 512]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

    }
}
