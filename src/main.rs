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
mod sidebar;

use crate::chessboard::Chessboard;
use crate::chessboard_controller::ChessboardController;
use crate::chessboard_view::{ChessboardView, ChessboardViewSettings};
use crate::sidebar::Sidebar;

pub const BOARD_SIZE: u8 = 8;
pub const BOARD_BORDER_SIZE: f64 = 5.0;
pub const WIDTH: f64 = 600.0;
pub const HEIGHT: f64 = 400.0 + 2.0 * BOARD_BORDER_SIZE;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Chess", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let chessboard = Chessboard::standard();
    let view_settings = { ChessboardViewSettings::new() };
    let view = ChessboardView::new(view_settings);
    let mut controller = ChessboardController::new(chessboard);
    controller.init_piece_rects();

    let sidebar_size = WIDTH - HEIGHT;
    //let mut sidebar_state = SidebarState { toggle: false };
    let mut sidebar = Sidebar::new(WIDTH - sidebar_size, 0.0, sidebar_size, HEIGHT);
    //sidebar.initialize(&mut sidebar_state);

    let mut cache = GlyphCache::new(
        "fonts/DejaVuSans.ttf",
        (),
        TextureSettings::new().filter(Filter::Linear),
    )
    .unwrap();

    // Create a new game and run it.
    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        controller.event(&e, &mut sidebar);
        sidebar.event(&e, &mut controller);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                use graphics::clear;
                clear([0.0; 4], gl);
                view.draw(&controller, &c, gl);
                sidebar.draw(&mut cache, &c.draw_state, c.transform, gl, &controller);
            });
        }
    }
}
