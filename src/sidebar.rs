use piston::input::GenericEvent;
use graphics;
use graphics::character::CharacterCache;
use opengl_graphics::{GlGraphics, Texture, TextureSettings, GlyphCache};


pub struct Sidebar {
    rect: [f64; 4],
}

impl Sidebar {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Sidebar {
        Sidebar {
            rect: [x, y, width, height],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) { 
    }


    pub fn draw(
        &self,
        c: &graphics::context::Context,
        g: &mut GlGraphics,
    ) {
        use graphics::{Rectangle, Text, Transformed};

        Rectangle::new([0.2, 0.2, 0.2, 1.0]).draw(
            self.rect,
            &c.draw_state,
            c.transform,
            g
        );
    }
}
