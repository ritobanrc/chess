use piston::input::GenericEvent;
use graphics;
use graphics::character::CharacterCache;
use graphics::{Graphics, Transformed, DrawState};
use graphics::math::Matrix2d;

use crate::chessboard_controller::Rectangle;

const TEXT_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];


pub struct Sidebar {
    rect: Rectangle,
}

impl Sidebar {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Sidebar {
        Sidebar {
            rect: Rectangle::new(x, y, width, height),
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) { 
    }


    pub fn draw<C, G>(&self,
                          cache: &mut C,
                          draw_state: &DrawState,
                          transform: Matrix2d,
                          g: &mut G)
            where C: CharacterCache,
                  G: Graphics<Texture = <C as CharacterCache>::Texture>
    {
        use graphics::{Rectangle, Text};

        let rect: [f64; 4] = self.rect.into();
        Rectangle::new([0.2, 0.2, 0.2, 1.0]).draw(
            rect,
            draw_state,
            transform,
            g
        );


        { // Player 1 Text
            let transform = transform.trans(self.rect.left() + 10.0, 
                                            self.rect.top() + 10.0 + 20.0);
            if let Ok(_) = Text::new_color(TEXT_COLOR, 20).
                draw("Player 1", cache, draw_state, transform, g) {
            } else { eprintln!("Error rendering text") }
        }

        { // Player 2 Text
            let transform = transform.trans(self.rect.left() + 10.0, 
                                            self.rect.bottom() - 10.0);
            if let Ok(_) = Text::new_color(TEXT_COLOR, 20).
                draw("Player 2", cache, draw_state, transform, g) {
            } else { eprintln!("Error rendering text") }
        }
    }
}

