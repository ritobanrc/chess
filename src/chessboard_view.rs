use graphics;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use std::path::Path;
use crate::ChessboardController;
use crate::piece::{Piece, Side};
use crate::{BOARD_SIZE};

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
    pub dark_pawn: Texture
}

impl ChesspieceTextures {
    pub fn from_paths(light_king_path: &String, 
                      light_queen_path: &String,
                      light_bishop_path: &String, 
                      light_knight_path: &String,
                      light_rook_path: &String,
                      light_pawn_path: &String,
                      dark_king_path: &String, 
                      dark_queen_path: &String,
                      dark_bishop_path: &String, 
                      dark_knight_path: &String,
                      dark_rook_path: &String,
                      dark_pawn_path: &String) -> ChesspieceTextures {

        ChesspieceTextures {
            light_king: Texture::from_path(Path::new(light_king_path), &TextureSettings::new()).unwrap(),
            light_queen: Texture::from_path(Path::new(light_queen_path), &TextureSettings::new()).unwrap(),
            light_bishop: Texture::from_path(Path::new(light_bishop_path), &TextureSettings::new()).unwrap(),
            light_knight: Texture::from_path(Path::new(light_knight_path), &TextureSettings::new()).unwrap(),
            light_rook: Texture::from_path(Path::new(light_rook_path), &TextureSettings::new()).unwrap(),
            light_pawn: Texture::from_path(Path::new(light_pawn_path), &TextureSettings::new()).unwrap(),
            dark_king: Texture::from_path(Path::new(dark_king_path), &TextureSettings::new()).unwrap(),
            dark_queen: Texture::from_path(Path::new(dark_queen_path), &TextureSettings::new()).unwrap(),
            dark_bishop: Texture::from_path(Path::new(dark_bishop_path), &TextureSettings::new()).unwrap(),
            dark_knight: Texture::from_path(Path::new(dark_knight_path), &TextureSettings::new()).unwrap(),
            dark_rook: Texture::from_path(Path::new(dark_rook_path), &TextureSettings::new()).unwrap(),
            dark_pawn: Texture::from_path(Path::new(dark_pawn_path), &TextureSettings::new()).unwrap(),
        }
    }

    pub fn get_piece_texture(&self, piece: &Piece) -> &Texture {
        match piece {
            Piece::King(data) => if data.side == Side::Light { &self.light_king } else { &self.dark_king }
            Piece::Queen(data) => if data.side == Side::Light { &self.light_queen } else { &self.dark_queen }
            Piece::Bishop(data) => if data.side == Side::Light { &self.light_bishop } else { &self.dark_bishop }
            Piece::Knight(data) => if data.side == Side::Light { &self.light_knight } else { &self.dark_knight }
            Piece::Rook(data) => if data.side == Side::Light { &self.light_rook } else { &self.dark_rook }
            Piece::Pawn(data) => if data.side == Side::Light { &self.light_pawn } else { &self.dark_pawn }
        }
    }
}

pub struct ChessboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
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
            &String::from("sprites/light_king.png"),
            &String::from("sprites/light_queen.png"),
            &String::from("sprites/light_bishop.png"),
            &String::from("sprites/light_knight.png"),
            &String::from("sprites/light_rook.png"),
            &String::from("sprites/light_pawn.png"),
            &String::from("sprites/dark_king.png"),
            &String::from("sprites/dark_queen.png"),
            &String::from("sprites/dark_bishop.png"),
            &String::from("sprites/dark_knight.png"),
            &String::from("sprites/dark_rook.png"),
            &String::from("sprites/dark_pawn.png")
            );
        ChessboardViewSettings {
            position: [5.0; 2],
            size: 800.0,
            light_square_color: [0.961, 0.961, 0.863, 1.0],
            dark_square_color: [0.545, 0.271, 0.075, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_size: 6.0,
            textures: textures,
        }
    }
}

pub struct ChessboardView {
    pub settings: ChessboardViewSettings,
}

impl ChessboardView {
    pub fn new(settings: ChessboardViewSettings) -> ChessboardView {
        ChessboardView {
            settings: settings,
        }
    }

    pub fn draw(&self, controller: &ChessboardController, c: &graphics::context::Context, g: &mut GlGraphics) {
        use graphics::{Rectangle, Image};

        let ref settings = self.settings;

        let square_size: f64 = settings.size/(BOARD_SIZE as f64);

        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                let square_rect = [settings.position[0] + (x as f64) * square_size, 
                    settings.position[1] + (y as f64) * square_size, 
                    square_size, square_size];
                let color = if x % 2 == y % 2 { settings.light_square_color } else { settings.dark_square_color };
                Rectangle::new(color).draw(square_rect, &c.draw_state, c.transform, g);
            }
        }

        let board_rect = [settings.position[0], settings.position[1], settings.size, settings.size];


        Rectangle::new_border(settings.board_edge_color, settings.board_edge_size)
            .draw(board_rect, &c.draw_state, c.transform, g);


        for (pos, piece) in controller.chessboard.get_pieces() {
            let img = Image::new().rect([settings.position[0] + pos[0] as f64 * square_size,
                                        (settings.position[1] + settings.size - square_size) - (pos[1] as f64 * square_size),
                                        square_size, square_size]);
            img.draw(settings.textures.get_piece_texture(piece), &c.draw_state, c.transform, g);
        }
    }
}
