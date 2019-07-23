use graphics;
use graphics::types::Color;
use graphics::{Graphics, Context, ImageSize};
//use graphics::context::Context;
use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use std::path::Path;
use crate::ChessboardController;
use crate::piece::{Piece, Side};
use crate::{BOARD_SIZE};

pub struct ChesspieceTextures {
    pub light_king: Texture,
}

impl ChesspieceTextures {
    pub fn from_paths(light_king_path: &String) -> ChesspieceTextures {
        ChesspieceTextures {
            light_king: Texture::from_path(Path::new(light_king_path), &TextureSettings::new()).unwrap(),
        }
    }

    pub fn get_piece_texture(&self, piece: &Piece) -> &Texture {
        match piece {
            Piece::King(data) => if data.side == Side::Light { &self.light_king } else { &self.light_king }
            _ => panic!("Not supported yet")
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
            &String::from("sprites/light_king.png")
            );
        ChessboardViewSettings {
            position: [6.0; 2],
            size: 500.0,
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

fn draw_texture<G, T>(c: &Context, g: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize {
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


        // Pieces
        let img = Image::new().rect([200.0, 200.0, 200.0, 200.0]);
        //let texture = opengl_graphics::Texture::from_path(Path::new("Example.png"), &TextureSettings::new()).unwrap();
        //image(&texture, c.transform, g)

        //img.draw(&texture, &c.draw_state, c.transform, g);
        img.draw(settings.textures.get_piece_texture(&controller.chessboard.get_piece_at("e1").unwrap()), &c.draw_state, c.transform, g);
    }
}
