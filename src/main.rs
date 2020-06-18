use sdl2::{event::Event, keyboard::Keycode, mouse::Mouse, rect::Point};

use sdl2_image::INIT_PNG;

use std::{thread, time::Duration};

mod board;
use board::ChessBoard;

mod drawable;

mod sdl_handle;
use sdl_handle::SDLHandle;

mod sprite;

mod utils;

fn main() {
    println!("Hello, world!");

    let mut sdl_handle = SDLHandle::init("Chess SDL2", (800, 600), INIT_PNG).unwrap();

    let sprites =
        sprite::load_grid_sprite_sheet(&sdl_handle, sdl_handle.asset_path("sprite_sheet.png"), 32)
            .unwrap();
    let board = ChessBoard::new(sprites);

    let mut events = sdl_handle.event_pump().unwrap();
    let mut board_center = Point::from(utils::map_tuple(sdl_handle.center_of_draw(), |val| {
        use std::convert::TryFrom;
        i32::try_from(val).unwrap()
    }));

    'run_loop: loop {
        while let Some(event) = events.poll_event() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'run_loop,
                Event::MouseButtonDown {
                    mouse_btn: Mouse::Left,
                    x,
                    y,
                    ..
                } => {
                    let in_board = board_center - (x, y).into();
                    if let Some(square) = ChessBoard::tile_coord(in_board) {
                        println!("{}", square);
                    }
                }
                _ => {}
            }
        }

        sdl_handle.clear();
        sdl_handle.draw_at(board_center, &board).unwrap();
        sdl_handle.present();

        thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }

    println!("Shutting down");
}
