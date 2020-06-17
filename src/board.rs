use chess::{Board, File, Rank, Square, NUM_FILES, NUM_RANKS, Color};

use sdl2::{rect::Rect, render::Renderer};

use std::convert::TryInto;

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

    pub fn draw_on(&self, dest: &mut Renderer, target_area: Option<Rect>) -> Result<(), String> {
        for x in 0..NUM_FILES {
            for y in 0..NUM_RANKS {
                let square = Square::make_square(Rank::from_index(y), File::from_index(x));

                let (x, y): (i32, _) = utils::map_tuple((x, y), |val| val.try_into().unwrap());
                let tile_size : i32 = TILE_SIZE.into();

                let rect = Rect::new(
                    x * tile_size,
                    y * tile_size,
                    TILE_SIZE.into(),
                    TILE_SIZE.into(),
                );

                if (x + y) % 2 == 0 {
                // White square
                    self.sprites[0].draw_on(dest, Some(rect))?;
                } else {
                // Black square
                    self.sprites[1].draw_on(dest, Some(rect))?;
                }

                if let Some(piece) = self.board.piece_on(square) {
                    let color = self.board.color_on(square).unwrap();

                    use Color::*;
                    use chess::Piece::*;
                    match (piece, color) {
                        (Pawn, Black) => self.sprites[2].draw_on(dest, Some(rect))?,
                        (Pawn, White) => self.sprites[3].draw_on(dest, Some(rect))?,
                        _ => eprintln!("Unimplemented piece {:?} {:?}", piece, color),
                    }
                }
            }
        }
        Ok(())
    }
}
