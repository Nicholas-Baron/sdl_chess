use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, rect::Point};

use sdl2::image::InitFlag;

use std::{thread, time::Duration};

mod ai;

mod board;
use board::ChessBoard;

mod drawable;

mod sdl_handle;
use sdl_handle::SDLHandle;

mod sprite;

mod twc;

mod utils;

fn initial_board_center(center: (u32, u32)) -> Point {
    Point::from(utils::map_tuple(center, |val| {
        use std::convert::TryFrom;
        i32::try_from(val).unwrap()
    }))
}

fn draw_board(sdl_handle: &mut SDLHandle, board: ChessBoard, board_center: Point) {
    sdl_handle.clear();
    sdl_handle.draw_at(board_center, &board).unwrap();
    sdl_handle.present();
}

fn main() {
    println!("Hello, world!");

    let mut sdl_handle = SDLHandle::init("Chess SDL2", (800, 600), InitFlag::PNG).unwrap();
    let mut events = sdl_handle.event_pump().unwrap();
    let mut board_center = initial_board_center(sdl_handle.center_of_draw());

    {
        let sprite_sheet_path = sdl_handle.asset_path("sprite_sheet.png");
        let texture_creator = sdl_handle.texture_creator();
        let sprites =
            sprite::load_grid_sprite_sheet(&texture_creator, sprite_sheet_path, 32).unwrap();
        let mut board = ChessBoard::new(sprites);

        'run_loop: loop {
            while let Some(event) = events.poll_event() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'run_loop,
                    Event::MouseButtonDown {
                        mouse_btn: MouseButton::Left,
                        x,
                        y,
                        ..
                    } => {
                        let in_board = Point::new(x - board_center.x(), board_center.y() - y);
                        board.select(ChessBoard::tile_coord(in_board));
                    }
                    Event::KeyDown {
                        keycode: Some(key), ..
                    } => match key {
                        Keycode::Left => board_center = board_center.offset(-5, 0),
                        Keycode::Right => board_center = board_center.offset(5, 0),
                        Keycode::Up => board_center = board_center.offset(0, -5),
                        Keycode::Down => board_center = board_center.offset(0, 5),
                        _ => {}
                    },
                    _ => {}
                }
            }

            draw_board(&mut sdl_handle, board.clone(), board_center);

            if board.is_ongoing() {
                board.try_resolve_ai();
            } else if board.is_player_winner() {
                println!("Player won");
            } else {
                println!("Stalemate or AI won");
            }

            thread::sleep(Duration::new(0, 1_000_000_000 / 60));
        }
    }
    println!("Shutting down");
}
