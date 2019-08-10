extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

#[macro_use] extern crate conrod_core;
extern crate conrod_piston;

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

pub const BOARD_SIZE: u8 = 8;
pub const BOARD_BORDER_SIZE: f64 = 5.0;
pub const WIDTH: f64 = 600.0;
pub const HEIGHT: f64 = 450.0 + 2.0*BOARD_BORDER_SIZE;

widget_ids! {
    pub struct Ids {
        canvas,
        title
    }
}

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

    let mut ui = conrod_core::UiBuilder::new([WIDTH, HEIGHT]).build();
    let image_map = conrod_core::image::Map::<opengl_graphics::Texture>::new();
    let ids = Ids::new(ui.widget_id_generator());

    let chessboard = Chessboard::standard();
    let view_settings = { ChessboardViewSettings::new() };
    let view = ChessboardView::new(view_settings);
    let mut controller = ChessboardController::new(chessboard);
    controller.init_piece_rects();

    // Create a new game and run it.
    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new().lazy(true));
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                use graphics::clear;
                clear([0.0; 4], gl);
                view.draw(&controller, &c, gl);
                if let Some(primitives) = ui.draw_if_changed() {
                    conrod_piston::draw::primitives(primitives,
                                                        c,
                                                        gl,
                                                        /*what*/);
                }
            });
        }

        if let Some(e) = conrod_piston::event::convert(e.clone(), WIDTH, HEIGHT) {
            ui.handle_event(e);
        }

        e.update(|_| {
            use conrod_core::{widget, Colorable, Labelable, Positionable, Sizeable, Widget};
            let mut ui = ui.set_widgets();

            const TITLE: &'static str = "All Widgets";
            const MARGIN: conrod_core::Scalar = 30.0;
            const TITLE_SIZE: conrod_core::FontSize = 42;
            widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, &mut ui);
            widget::Text::new(TITLE).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, &mut ui);
        });

        controller.event(&e);
    }
}
