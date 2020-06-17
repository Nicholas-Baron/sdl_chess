use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::RendererBuilder,
};

use sdl2_image::{self as image, INIT_PNG};

use std::{env::current_dir, thread, time::Duration};

mod board;
use board::ChessBoard;

mod sprite;

mod utils;

fn main() {
    println!("Hello, world!");

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video
        .window("Rust SDL2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.renderer().present_vsync().build().unwrap();

    let _image_context = image::init(INIT_PNG).unwrap();

    let mut app_dir = current_dir().unwrap();
    app_dir.push("assets/sprite_sheet.png");
    let sprites = sprite::load_grid_sprite_sheet(&canvas, app_dir, 32).unwrap();
    let board = ChessBoard::new(sprites);

    canvas.set_draw_color(Color::RGB(0, 250, 250));
    if let Err(e) = canvas.set_logical_size(800, 600) {
        eprintln!("{}", e);
    }
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump().unwrap();
    let mut board_center = Point::from(utils::map_tuple(canvas.logical_size(), |val| {
        use std::convert::TryFrom;
        i32::try_from(val / 2).unwrap()
    }));

    'run_loop: loop {

        while let Some(event) = events.poll_event() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'run_loop,
                _ => {}
            }
        }

        canvas.clear();
        board.draw_on(&mut canvas, Some(board_center)).unwrap();
        canvas.present();

        thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }

    println!("Shutting down");
}
