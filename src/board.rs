use chess::{Board, Color, File, Rank, Square, NUM_FILES, NUM_RANKS};

use sdl2::{
    rect::{Point, Rect},
    render::Renderer,
};

use std::convert::{TryFrom, TryInto};

use crate::{sprite::Sprite, utils};

pub struct ChessBoard {
    board: Board,
    sprites: Vec<Sprite>,
}

const TILE_SIZE: u8 = 32;

impl ChessBoard {
    pub fn new(sprites: Vec<Sprite>) -> Self {
        Self {
            board: Default::default(),
            sprites,
        }
    }

    pub fn draw_on(&self, dest: &mut Renderer, center: Option<Point>) -> Result<(), String> {
        for x in 0..NUM_FILES {
            for y in 0..NUM_RANKS {
                let square = Square::make_square(Rank::from_index(y), File::from_index(x));

                let (x, y): (i32, _) = utils::map_tuple((x, y), |val| val.try_into().unwrap());
                let tile_size: i32 = TILE_SIZE.into();

                let (mut pixel_x, mut pixel_y) = utils::map_tuple((x, y), |val| val * tile_size);

                if let Some(offset) = center {
                    let board_size: i32 = i32::try_from(NUM_FILES).unwrap() * tile_size;
                    pixel_x += offset.x() - board_size / 2;
                    pixel_y += offset.y() - board_size / 2;
                }

                let rect = Rect::new(pixel_x, pixel_y, TILE_SIZE.into(), TILE_SIZE.into());

                if (x + y) % 2 == 0 {
                    // White square
                    self.sprites[0].draw_on(dest, Some(rect))?;
                } else {
                    // Black square
                    self.sprites[1].draw_on(dest, Some(rect))?;
                }

                if let Some(piece) = self.board.piece_on(square) {
                    let color = self.board.color_on(square).unwrap();

                    use chess::Piece::*;
                    use Color::*;
                    match (piece, color) {
                        (Pawn, Black) => self.sprites[2].draw_on(dest, Some(rect))?,
                        (Pawn, White) => self.sprites[3].draw_on(dest, Some(rect))?,
                        (Rook, Black) => self.sprites[4].draw_on(dest, Some(rect))?,
                        (Rook, White) => self.sprites[5].draw_on(dest, Some(rect))?,
                        _ => eprintln!("Unimplemented piece {:?} {:?}", piece, color),
                    }
                }
            }
        }
        Ok(())
    }
}
