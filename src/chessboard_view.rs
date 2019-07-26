use crate::piece::{Piece, Side};
use crate::ChessboardController;
use crate::BOARD_SIZE;
use graphics;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use std::path::Path;

pub struct ChesspieceTextures {
    pub light_king: Texture,
    pub light_queen: Texture,
    pub light_bishop: Texture,
    pub light_knight: Texture,
    pub light_rook: Texture,
    pub light_pawn: Texture,

    pub dark_king: Texture,
    pub dark_queen: Texture,
    pub dark_bishop: Texture,
    pub dark_knight: Texture,
    pub dark_rook: Texture,
    pub dark_pawn: Texture,
}

impl ChesspieceTextures {
    pub fn from_paths(
        light_king_path: &str,
        light_queen_path: &str,
        light_bishop_path: &str,
        light_knight_path: &str,
        light_rook_path: &str,
        light_pawn_path: &str,
        dark_king_path: &str,
        dark_queen_path: &str,
        dark_bishop_path: &str,
        dark_knight_path: &str,
        dark_rook_path: &str,
        dark_pawn_path: &str,
    ) -> ChesspieceTextures {
        ChesspieceTextures {
            light_king: Texture::from_path(Path::new(light_king_path), &TextureSettings::new())
                .unwrap(),
            light_queen: Texture::from_path(Path::new(light_queen_path), &TextureSettings::new())
                .unwrap(),
            light_bishop: Texture::from_path(Path::new(light_bishop_path), &TextureSettings::new())
                .unwrap(),
            light_knight: Texture::from_path(Path::new(light_knight_path), &TextureSettings::new())
                .unwrap(),
            light_rook: Texture::from_path(Path::new(light_rook_path), &TextureSettings::new())
                .unwrap(),
            light_pawn: Texture::from_path(Path::new(light_pawn_path), &TextureSettings::new())
                .unwrap(),
            dark_king: Texture::from_path(Path::new(dark_king_path), &TextureSettings::new())
                .unwrap(),
            dark_queen: Texture::from_path(Path::new(dark_queen_path), &TextureSettings::new())
                .unwrap(),
            dark_bishop: Texture::from_path(Path::new(dark_bishop_path), &TextureSettings::new())
                .unwrap(),
            dark_knight: Texture::from_path(Path::new(dark_knight_path), &TextureSettings::new())
                .unwrap(),
            dark_rook: Texture::from_path(Path::new(dark_rook_path), &TextureSettings::new())
                .unwrap(),
            dark_pawn: Texture::from_path(Path::new(dark_pawn_path), &TextureSettings::new())
                .unwrap(),
        }
    }

    pub fn get_piece_texture(&self, piece: &Piece) -> &Texture {
        match piece {
            Piece::King(data) => {
                if data.side == Side::Light {
                    &self.light_king
                } else {
                    &self.dark_king
                }
            }
            Piece::Queen(data) => {
                if data.side == Side::Light {
                    &self.light_queen
                } else {
                    &self.dark_queen
                }
            }
            Piece::Bishop(data) => {
                if data.side == Side::Light {
                    &self.light_bishop
                } else {
                    &self.dark_bishop
                }
            }
            Piece::Knight(data) => {
                if data.side == Side::Light {
                    &self.light_knight
                } else {
                    &self.dark_knight
                }
            }
            Piece::Rook(data) => {
                if data.side == Side::Light {
                    &self.light_rook
                } else {
                    &self.dark_rook
                }
            }
            Piece::Pawn(data) => {
                if data.side == Side::Light {
                    &self.light_pawn
                } else {
                    &self.dark_pawn
                }
            }
        }
    }
}

pub struct ChessboardViewSettings {
    /// Color of the light squares
    pub light_square_color: Color,
    /// Color of the dark squares
    pub dark_square_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_size: f64,
    /// Struct storing the textures for the chesspieces
    pub textures: ChesspieceTextures,
}

impl ChessboardViewSettings {
    pub fn new() -> ChessboardViewSettings {
        let textures = ChesspieceTextures::from_paths(
            "sprites/light_king.png",
            "sprites/light_queen.png",
            "sprites/light_bishop.png",
            "sprites/light_knight.png",
            "sprites/light_rook.png",
            "sprites/light_pawn.png",
            "sprites/dark_king.png",
            "sprites/dark_queen.png",
            "sprites/dark_bishop.png",
            "sprites/dark_knight.png",
            "sprites/dark_rook.png",
            "sprites/dark_pawn.png",
        );
        ChessboardViewSettings {
            light_square_color: [0.961, 0.961, 0.863, 1.0],
            dark_square_color: [0.545, 0.271, 0.075, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_size: 5.0,
            textures,
        }
    }
}

pub struct ChessboardView {
    pub settings: ChessboardViewSettings,
}

impl ChessboardView {
    #[inline(always)]
    pub fn new(settings: ChessboardViewSettings) -> ChessboardView {
        ChessboardView { settings: settings }
    }

    pub fn draw(
        &self,
        controller: &ChessboardController,
        c: &graphics::context::Context,
        g: &mut GlGraphics,
    ) {
        use graphics::{Image, Rectangle};
        let settings = &self.settings;

        let square_size: f64 = controller.size / (BOARD_SIZE as f64);

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let square_rect = [
                    controller.position[0] + (x as f64) * square_size,
                    controller.position[1] + (y as f64) * square_size,
                    square_size,
                    square_size,
                ];
                let color = if x % 2 == y % 2 {
                    settings.light_square_color
                } else {
                    settings.dark_square_color
                };
                Rectangle::new(color).draw(square_rect, &c.draw_state, c.transform, g);
            }
        }

        let board_rect = [
            controller.position[0],
            controller.position[1],
            controller.size,
            controller.size,
        ];

        Rectangle::new_border(settings.board_edge_color, settings.board_edge_size).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );

        //for (_, piece) in controller.chessboard.get_pieces() {
        for piece_rect in &controller.piece_rects {
            let img: Image = (&piece_rect.rect).into();
            img.draw(
                settings.textures.get_piece_texture(piece_rect.piece),
                &c.draw_state,
                c.transform,
                g,
            );
        }
    }
}
