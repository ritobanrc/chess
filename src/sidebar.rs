use piston::input::GenericEvent;
use graphics;
use graphics::character::CharacterCache;
use graphics::{Graphics, Transformed, DrawState};
use graphics::math::Matrix2d;

use crate::chessboard_controller::Rectangle;

const TEXT_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];


pub struct Sidebar<'a> {
    rect: Rectangle,
    buttons: Vec<Button<'a>>,
}

pub struct SidebarState {
    pub toggle: bool,
}

impl<'a> Sidebar<'a> {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Sidebar<'a> {
        Sidebar {
            rect: Rectangle::new(x, y, width, height),
            buttons: Vec::new(),
        }
    }

    pub fn initialize(&mut self, sidebar_state: &'a mut SidebarState) { 
        let theme = ButtonTheme::default();
        let mut toggle1 = false;
        let mut toggle2 = false;
        self.buttons.push(Button::new(Rectangle::from([10.0, 50.0, 25.0, 25.0]), theme,
                          move || {
                              toggle1 = !toggle1;
                              println!("button pressed {:?}", toggle1);
                              //sidebar_state.toggle = !sidebar_state.toggle;
                          }
                          ));
        self.buttons.push(Button::new(Rectangle::from([40.0, 50.0, 25.0, 25.0]), theme,
                          move || {
                              toggle2 = !toggle2;
                              println!("other button pressed {:?}", toggle2);
                              //sidebar_state.toggle = !sidebar_state.toggle;
                          }
                          ));
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) { 
        for button in &mut self.buttons {
            button.event(e, [self.rect.left(), self.rect.top()]);
        }
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

        // Background
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

        for button in &self.buttons {
            let transform = transform.trans(self.rect.left(), self.rect.top());
            button.draw(cache, draw_state, transform, g);
        }

        //{ // Rectangle
            //let transform = transform.trans(self.rect.center_x(), 
                                            //self.rect.center_y());
            //Rectangle::new([0.2, 0.2, 0.8, 1.0]).draw([10.0, 10.0, 50.0, 20.0], draw_state, transform, g);
        //}
    }
}

#[derive(Clone, Copy)]
struct ButtonTheme {
    base_color: [f32; 4],
    text_color: [f32; 4],
    hover_color: [f32; 4],
    pressed_color: [f32; 4],
}

impl ButtonTheme {
    pub fn default() -> Self {
        ButtonTheme {
            base_color: [0.1, 0.1, 0.1, 1.0],
            text_color: [0.9, 0.9, 0.9, 1.0],
            hover_color: [0.1, 0.1, 0.2, 1.0],
            pressed_color: [0.1, 0.1, 0.3, 1.0],
        }
    }
}

struct Button<'a> { 
    rect: Rectangle,
    theme: ButtonTheme,
    //text: String,
    hover: bool,
    pressed: bool,
    callback: Box<dyn 'a + FnMut()>
}

impl<'a> Button<'a> {
    pub fn new<CB: 'a + FnMut()>(rect: Rectangle, theme: ButtonTheme, callback: CB) -> Self { 
        Button {
            rect,
            theme,
            hover: false,
            pressed: false,
            callback: Box::new(callback),
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E, offset: [f64; 2]) { 
        e.mouse_cursor(|pos| {
            if self.rect.offset(offset).is_point_inside(pos[0], pos[1]) {
                self.hover = true;
            } else {
                self.hover = false;
                self.pressed = false;
            }
        });

        e.press(|button| {
            if self.hover {
                match button {
                    piston::Button::Mouse(piston::MouseButton::Left) => {
                        self.pressed = true;
                    }
                    _ => { }
                }
            }
        });

        e.release(|button| {
            if self.hover && self.pressed {
                match button {
                    piston::Button::Mouse(piston::MouseButton::Left) => {
                        self.pressed = false;
                        (self.callback)();
                    }
                    _ => { }
                }
            }
        });
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
        let mut background_color = self.theme.base_color;
        if self.pressed {
            background_color = self.theme.pressed_color;
        }
        else if self.hover {
            background_color = self.theme.hover_color;
        };
        Rectangle::new(background_color).draw(self.rect, draw_state, transform, g);
    }
}
