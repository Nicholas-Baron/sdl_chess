use chess::{Board, Color, File, Rank, Square, NUM_FILES};

use sdl2::{
    pixels,
    rect::{Point, Rect},
    render::Renderer,
};

use std::convert::{TryFrom, TryInto};

use crate::{drawable::Drawable, sprite::Sprite, utils};

pub struct ChessBoard {
    board: Board,
    sprites: Vec<Sprite>,
    selected_square: Option<Square>,
}

const TILE_SIZE: u8 = 32;

impl ChessBoard {
    pub fn new(sprites: Vec<Sprite>) -> Self {
        chess::ALL_SQUARES
            .iter()
            .map(|&square| (square, chess::BoardBuilder::from(Board::default())[square]))
            .filter_map(|(square, piece)| match piece {
                None => None,
                Some((piece, color)) => Some((square, color, piece)),
            })
            .for_each(|(square, color, piece)| {
                println!(
                    "{} (aka {}) has {:?} {:?}",
                    square,
                    square.to_int(),
                    color,
                    piece
                )
            });

        Self {
            board: Default::default(),
            sprites,
            selected_square: Default::default(),
        }
    }

    /// Checks if the pixel position (relative to the center of the board) is inside the board
    pub fn contains_from_center(p: Point) -> bool {
        let board_size: u32 = Self::board_size().try_into().unwrap();
        Rect::from_center((0, 0), board_size, board_size).contains(p)
    }

    /// Returns the square corresponding to the given point (relative from the center)
    pub fn tile_coord(p: Point) -> Option<Square> {
        if !Self::contains_from_center(p) {
            None
        } else {
            let half_board_size = Self::board_size() / 2;

            let pixel_pos = p + (half_board_size, half_board_size).into();
            let tile_pos = pixel_pos / TILE_SIZE.into();

            let (tile_x, tile_y) =
                utils::map_tuple(tile_pos.into(), |val| usize::try_from(val).unwrap());

            Some(Square::make_square(
                Rank::from_index(tile_y),
                File::from_index(tile_x),
            ))
        }
    }

    pub fn select(&mut self, square: Option<Square>) {
        self.selected_square = square;
        if let Some(square) = square {
            println!("Selected {}", square);
        }
    }

    /// The board size in pixels
    fn board_size() -> i32 {
        let tile_size: i32 = TILE_SIZE.into();
        i32::try_from(NUM_FILES).unwrap() * tile_size
    }

    fn draw_position(square: Square, center: Point) -> Rect {
        let tile_pos = (square.get_file().to_index(), square.get_rank().to_index());
        let (x, y): (i32, _) = utils::map_tuple(tile_pos, |val| val.try_into().unwrap());

        let tile_size: i32 = TILE_SIZE.into();
        let (mut pixel_x, mut pixel_y) = utils::map_tuple((x, y), |val| val * tile_size);

        let half_board_size = Self::board_size() / 2;
        pixel_x += center.x() - half_board_size;

        pixel_y = -pixel_y + (half_board_size - tile_size);
        pixel_y += center.y();

        Rect::new(pixel_x, pixel_y, TILE_SIZE.into(), TILE_SIZE.into())
    }
}

impl Drawable for ChessBoard {
    fn draw_at(&self, dest: &mut Renderer, center: Point) -> Result<(), String> {
        for &square in chess::ALL_SQUARES.iter() {
            let rect = Self::draw_position(square, center);

            let in_board = Point::new(
                rect.center().x() - center.x(),
                center.y() - rect.center().y(),
            );

            assert_eq!(Self::tile_coord(in_board), Some(square));

            {
                let x = square.get_rank().to_index();
                let y = square.get_file().to_index();
                if (x + y) % 2 == 0 {
                    // White square
                    self.sprites[0].draw_on(dest, rect)?;
                } else {
                    // Black square
                    self.sprites[1].draw_on(dest, rect)?;
                }
            }

            if let Some(piece) = self.board.piece_on(square) {
                let color = self.board.color_on(square).unwrap();
                use chess::Piece::*;
                use Color::*;
                match (piece, color) {
                    (Pawn, Black) => self.sprites[2].draw_on(dest, rect)?,
                    (Pawn, White) => self.sprites[3].draw_on(dest, rect)?,
                    (Rook, Black) => self.sprites[4].draw_on(dest, rect)?,
                    (Rook, White) => self.sprites[5].draw_on(dest, rect)?,
                    _ => {} // eprintln!("Unimplemented piece {:?} {:?}", piece, color),
                }
            }

            if self
                .selected_square
                .map(|val| val == square)
                .unwrap_or(false)
            {
                // Something needs to be highlighted
                let magenta = pixels::Color::RGB(255, 0, 255);
                dest.set_draw_color(magenta);
                dest.draw_rect(rect)?;
            }
        }

        for (i, sprite) in self.sprites.iter().enumerate() {
            sprite.draw_on(
                dest,
                Rect::new(
                    (32 * i).try_into().unwrap(),
                    0,
                    TILE_SIZE.into(),
                    TILE_SIZE.into(),
                ),
            )?;
        }

        Ok(())
    }
}
