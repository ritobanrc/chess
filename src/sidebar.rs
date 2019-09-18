use std::collections::HashMap;
use piston::input::GenericEvent;
use graphics;
use graphics::character::CharacterCache;
use graphics::{Graphics, Transformed, DrawState};
use graphics::math::Matrix2d;

use crate::chessboard_controller::Rectangle;

const TEXT_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ButtonIds {
    Button1, Button2, Button3
}

pub struct Sidebar {
    rect: Rectangle,
    buttons: HashMap<ButtonIds, Button>,
    toggle: bool,
}

impl Sidebar {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Sidebar {
        let mut buttons = HashMap::new();
        let theme = ButtonTheme::default();
        buttons.insert(ButtonIds::Button1, Button::new(Rectangle::from([10.0, 50.0, 50.0, 50.0]), theme, "♙".to_string()));
        buttons.insert(ButtonIds::Button2, Button::new(Rectangle::from([70.0, 50.0, 100.0, 100.0]), theme, "♛".to_string()));
        Sidebar {
            rect: Rectangle::new(x, y, width, height),
            buttons: buttons,
            toggle: false
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) { 
        let mut add_button = false;
        let mut remove_button = false;
        for (id, button) in &mut self.buttons {
            let result = button.event(e, [self.rect.left(), self.rect.top()]);
            if result == ButtonStatus::Clicked {
                match id {
                    ButtonIds::Button1 => {
                        add_button = true;
                    }
                    ButtonIds::Button3 => {
                        remove_button = true;
                    }
                    _ => { 
                        self.toggle = !self.toggle;
                        println!("Button 1 Pressed");
                    }
                }
                println!("{}", self.toggle);
            }
        }
        if add_button {
            self.buttons.insert(ButtonIds::Button3, Button::new(Rectangle::from([20.0, 200.0, 25.0, 25.0]), ButtonTheme::default(), "f".to_string()));
        }
        if remove_button {
            self.buttons.remove(&ButtonIds::Button3);
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

        for (_, button) in &self.buttons {
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

struct Button { 
    rect: Rectangle,
    theme: ButtonTheme,
    text: String,
    hover: bool,
    pressed: bool,
}

#[derive(PartialEq, Eq)]
enum ButtonStatus {
    Nothing, Clicked,
}

impl Button {
    pub fn new(rect: Rectangle, theme: ButtonTheme, text: String) -> Self { 
        Button {
            rect,
            theme,
            text,
            hover: false,
            pressed: false,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E, offset: [f64; 2]) -> ButtonStatus { 
        let mut result = ButtonStatus::Nothing;

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
                        result = ButtonStatus::Clicked;
                        //(self.callback)();
                    }
                    _ => { }
                }
            }
        });

        result
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

        let transform = transform.trans(self.rect.center_x() - self.rect.size_x()/4.0,
                                        self.rect.center_y() + self.rect.size_y()/3.0);
        if let Ok(_) = Text::new_color(self.theme.text_color, (self.rect.size_y() - 5.0) as u32).
            draw(&self.text[..], cache, draw_state, transform, g) {
        } else { eprintln!("Error rendering text") }
    }
}
