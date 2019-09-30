use graphics;
use graphics::character::CharacterCache;
use graphics::math::Matrix2d;
use graphics::{DrawState, Graphics, Transformed};
use piston::input::GenericEvent;
use std::collections::HashMap;

use crate::chessboard_controller::{ChessboardController, Rectangle};
use crate::chessboard::Checkmate;
use crate::piece::{Piece, Side};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ButtonIds {
    RookButton,
    BishopButton,
    KnightButon,
    QueenButton,
}

pub struct Sidebar {
    rect: Rectangle,
    buttons: HashMap<ButtonIds, Button>,
}

impl Sidebar {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Sidebar {
        let buttons = HashMap::new();
        Sidebar {
            rect: Rectangle::new(x, y, width, height),
            buttons,
        }
    }

    pub fn add_pawn_buttons(&mut self) {
        let theme = ButtonTheme::default();
        self.buttons.insert(
            ButtonIds::RookButton,
            Button::new(
                Rectangle::from([10.0, 100.0, 75.0, 75.0]),
                theme,
                "♜".to_string(),
            ),
        );
        self.buttons.insert(
            ButtonIds::BishopButton,
            Button::new(
                Rectangle::from([100.0, 100.0, 75.0, 75.0]),
                theme,
                "♝".to_string(),
            ),
        );
        self.buttons.insert(
            ButtonIds::KnightButon,
            Button::new(
                Rectangle::from([10.0, 200.0, 75.0, 75.0]),
                theme,
                "♞".to_string(),
            ),
        );
        self.buttons.insert(
            ButtonIds::QueenButton,
            Button::new(
                Rectangle::from([100.0, 200.0, 75.0, 75.0]),
                theme,
                "♛".to_string(),
            ),
        );
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        e: &E,
        chessboard_controller: &mut ChessboardController,
    ) {
        //let mut add_button = false;
        let mut remove_pawn_buttons = false;
        for (id, button) in &mut self.buttons {
            let result = button.event(e, [self.rect.left(), self.rect.top()]);
            if result == ButtonStatus::Clicked {
                match id {
                    ButtonIds::QueenButton => {
                        chessboard_controller.trigger_pawn_promotion(&Piece::Queen);
                        remove_pawn_buttons = true;
                    }
                    ButtonIds::KnightButon => {
                        chessboard_controller.trigger_pawn_promotion(&Piece::Knight);
                        remove_pawn_buttons = true;
                    }
                    ButtonIds::BishopButton => {
                        chessboard_controller.trigger_pawn_promotion(&Piece::Bishop);
                        remove_pawn_buttons = true;
                    }
                    ButtonIds::RookButton => {
                        chessboard_controller.trigger_pawn_promotion(&Piece::Rook);
                        remove_pawn_buttons = true;
                    }
                }
            }
        }
        if remove_pawn_buttons {
            self.buttons.remove(&ButtonIds::QueenButton);
            self.buttons.remove(&ButtonIds::KnightButon);
            self.buttons.remove(&ButtonIds::BishopButton);
            self.buttons.remove(&ButtonIds::RookButton);
        }
    }

    pub fn draw<C, G>(&self, cache: &mut C, draw_state: &DrawState, transform: Matrix2d, g: &mut G, controller: &ChessboardController)
    where
        C: CharacterCache,
        G: Graphics<Texture = <C as CharacterCache>::Texture>,
    {
        use graphics::{Rectangle, Text};

        const TEXT_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];

        // Background
        let rect: [f64; 4] = self.rect.into();
        Rectangle::new([0.2, 0.2, 0.2, 1.0]).draw(rect, draw_state, transform, g);

        {
            // Black Text
            let size = 20;
            let transform = transform.trans(self.rect.left() + 10.0, self.rect.top() + 10.0 + f64::from(size));
            if Text::new_color(TEXT_COLOR, size)
                .draw("Black", cache, draw_state, transform, g)
                .is_err()
            {
                eprintln!("Error rendering text")
            }
        }

        {
            // White Text
            let transform = transform.trans(self.rect.left() + 10.0, self.rect.bottom() - 10.0);
            if Text::new_color(TEXT_COLOR, 20)
                .draw("White", cache, draw_state, transform, g)
                .is_err()
            {
                eprintln!("Error rendering text")
            }
        }

        {
            // Black Captured Pieces
            let size = 15;
            let transform = transform.trans(self.rect.left() + 10.0, self.rect.top() + f64::from(size) + 40.0);
            if Text::new_color(TEXT_COLOR, size)
                .draw(&controller.get_captures_for_side(Side::Dark).display(), cache, draw_state, transform, g)
                .is_err()
            {
                eprintln!("Error rendering text")
            }
        }

        {
            // White Captured Pieces
            let size = 15;
            let transform = transform.trans(self.rect.left() + 10.0, self.rect.bottom() - 40.0);
            if Text::new_color(TEXT_COLOR, size)
                .draw(&controller.get_captures_for_side(Side::Light).display(), cache, draw_state, transform, g)
                .is_err()
            {
                eprintln!("Error rendering text")
            }
        }

        // Black Check/Checkmate
        // Note that the game_result stores the winner, but we want to display "Checkmate" on the
        // loser
        if controller.game_result.0 == Checkmate::Checkmate && controller.game_result.1 == Side::Light {
            let transform = transform.trans(self.rect.center_x() - 7.0, self.rect.top() + 10.0 + 20.0);
            if Text::new_color(TEXT_COLOR, 15)
                .draw("Checkmate", cache, draw_state, transform, g)
                .is_err()
            {
                eprintln!("Error rendering text")
            }
        } else if controller.get_check_for_side(Side::Dark) {
            let transform = transform.trans(self.rect.center_x(), self.rect.top() + 10.0 + 20.0);
            if Text::new_color(TEXT_COLOR, 15)
                .draw("Check", cache, draw_state, transform, g)
                .is_err()
            {
                eprintln!("Error rendering text")
            }
        }


        // White Check/Checkmate
        // Note that the game_result stores the winner, but we want to display "Checkmate" on the
        // loser
        if controller.game_result.0 == Checkmate::Checkmate && controller.game_result.1 == Side::Dark {
            let transform = transform.trans(self.rect.center_x() - 7.0, self.rect.bottom() - 10.0);
            if Text::new_color(TEXT_COLOR, 15)
                .draw("Checkmate", cache, draw_state, transform, g)
                .is_err()
            {
                eprintln!("Error rendering text")
            }
        } else if controller.get_check_for_side(Side::Light) {
            let transform = transform.trans(self.rect.center_x(), self.rect.bottom() - 10.0);
            if Text::new_color(TEXT_COLOR, 15)
                .draw("Check", cache, draw_state, transform, g)
                .is_err()
            {
                eprintln!("Error rendering text")
            }
        }

        match controller.game_result {
            (Checkmate::Nothing, _) => { },
            (Checkmate::Checkmate, Side::Light) => {
                {
                    let transform = transform.trans(self.rect.center_x() - 80.0, self.rect.center_y() - 20.0);
                    if Text::new_color(TEXT_COLOR, 20)
                        .draw("GAME OVER!", cache, draw_state, transform, g)
                        .is_err()
                    {
                        eprintln!("Error rendering text")
                    }
                }
                {
                    let transform = transform.trans(self.rect.center_x() - 60.0, self.rect.center_y() + 20.0);
                    if Text::new_color(TEXT_COLOR, 15)
                        .draw("WHITE WINS", cache, draw_state, transform, g)
                        .is_err()
                    {
                        eprintln!("Error rendering text")
                    }
                }
            },
            (Checkmate::Checkmate, Side::Dark) => {
                {
                    let transform = transform.trans(self.rect.center_x() - 80.0, self.rect.center_y() - 20.0);
                    if Text::new_color(TEXT_COLOR, 20)
                        .draw("GAME OVER!", cache, draw_state, transform, g)
                        .is_err()
                    {
                        eprintln!("Error rendering text")
                    }
                }
                {
                    let transform = transform.trans(self.rect.center_x() - 60.0, self.rect.center_y() + 20.0);
                    if Text::new_color(TEXT_COLOR, 15)
                        .draw("BLACK WINS", cache, draw_state, transform, g)
                        .is_err()
                    {
                        eprintln!("Error rendering text")
                    }
                }
            },
            (Checkmate::Stalemate, _) => {
                {
                    let transform = transform.trans(self.rect.center_x() - 80.0, self.rect.center_y());
                    if Text::new_color(TEXT_COLOR, 15)
                        .draw("GAME OVER!", cache, draw_state, transform, g)
                        .is_err()
                    {
                        eprintln!("Error rendering text")
                    }
                }
                {
                    let transform = transform.trans(self.rect.center_x() - 30.0, self.rect.center_y() + 20.0);
                    if Text::new_color(TEXT_COLOR, 15)
                        .draw("DRAW", cache, draw_state, transform, g)
                        .is_err()
                    {
                        eprintln!("Error rendering text")
                    }
                }
            }
        }


        for button in self.buttons.values() {
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
    Nothing,
    Clicked,
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
                if let piston::Button::Mouse(piston::MouseButton::Left) = button {
                    self.pressed = true;
                }
            }
        });

        e.release(|button| {
            if self.hover && self.pressed {
                if let piston::Button::Mouse(piston::MouseButton::Left) = button {
                    self.pressed = false;
                    result = ButtonStatus::Clicked;
                }
            }
        });

        result
    }

    pub fn draw<C, G>(&self, cache: &mut C, draw_state: &DrawState, transform: Matrix2d, g: &mut G)
    where
        C: CharacterCache,
        G: Graphics<Texture = <C as CharacterCache>::Texture>,
    {
        use graphics::{Rectangle, Text};
        let mut background_color = self.theme.base_color;
        if self.pressed {
            background_color = self.theme.pressed_color;
        } else if self.hover {
            background_color = self.theme.hover_color;
        };
        Rectangle::new_round(background_color, 5.0).draw(self.rect, draw_state, transform, g);

        let transform = transform.trans(
            self.rect.center_x() - self.rect.size_x() / 2.0,
            self.rect.center_y() + self.rect.size_y() / 2.0,
        );

        if Text::new_color(self.theme.text_color, (self.rect.size_y() * 0.9) as u32)
            .draw(&self.text[..], cache, draw_state, transform, g)
            .is_err()
        {
            eprintln!("Error rendering text")
        }
    }
}
