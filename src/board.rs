use chess::{Board, Color, File, Rank, Square, NUM_FILES, NUM_RANKS};

use sdl2::{
    rect::{Point, Rect},
    render::Renderer,
};

use std::convert::{TryFrom, TryInto};

use crate::{drawable::Drawable, sprite::Sprite, utils};

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

    /// Checks if the pixel position (relative to the center of the board) is inside the board
    pub fn contains_from_center(p: Point) -> bool {
        let (x, y) = p.into();
        let half_board_size = Self::board_size() / 2;
        x > -half_board_size && x < half_board_size && y > -half_board_size && y < half_board_size
    }

    /// Returns the square corresponding to the given point (relative from the center)
    pub fn tile_coord(p: Point) -> Option<Square> {
        if !Self::contains_from_center(p) {
            None
        } else {
            let half_board_size = Self::board_size() / 2;

            let tile_pos = (p + (half_board_size, half_board_size).into()) / TILE_SIZE.into();
            println!("Tile pos: {:?}", tile_pos);

            // We calculate from the top left, however chess notation starts at the bottom left

            let correct_x = -tile_pos.x() + 7;

            let (tile_x, tile_y) = utils::map_tuple((correct_x, tile_pos.y()), |val| {
                usize::try_from(val).unwrap()
            });

            Some(Square::make_square(
                Rank::from_index(tile_y),
                File::from_index(tile_x),
            ))
        }
    }

    /// The board size in pixels
    fn board_size() -> i32 {
        let tile_size: i32 = TILE_SIZE.into();
        i32::try_from(NUM_FILES).unwrap() * tile_size
    }
}

impl Drawable for ChessBoard {
    fn draw_at(&self, dest: &mut Renderer, center: Point) -> Result<(), String> {
        for x in 0..NUM_FILES {
            for y in 0..NUM_RANKS {
                let square = Square::make_square(Rank::from_index(y), File::from_index(x));

                let (x, y): (i32, _) = utils::map_tuple((x, y), |val| val.try_into().unwrap());

                let tile_size: i32 = TILE_SIZE.into();
                let (mut pixel_x, mut pixel_y) = utils::map_tuple((x, y), |val| val * tile_size);

                let board_size = Self::board_size();
                pixel_x += center.x() - board_size / 2;
                pixel_y += center.y() - board_size / 2;

                let rect = Rect::new(pixel_x, pixel_y, TILE_SIZE.into(), TILE_SIZE.into());

                if (x + y) % 2 == 0 {
                    // White square
                    self.sprites[0].draw_on(dest, rect)?;
                } else {
                    // Black square
                    self.sprites[1].draw_on(dest, rect)?;
                }

                if let Some(piece) = self.board.piece_on(square) {
                    let color = self.board.color_on(square).unwrap();

                    use chess::Piece::*;
                    use Color::*;
                    match (piece, color) {
                        (Pawn, White) => self.sprites[2].draw_on(dest, rect)?,
                        (Pawn, Black) => self.sprites[3].draw_on(dest, rect)?,
                        (Rook, White) => self.sprites[4].draw_on(dest, rect)?,
                        (Rook, Black) => self.sprites[5].draw_on(dest, rect)?,
                        _ => {} // eprintln!("Unimplemented piece {:?} {:?}", piece, color),
                    }
                }
            }
        }
        Ok(())
    }
}
