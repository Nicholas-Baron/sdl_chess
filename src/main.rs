use sdl2::{event::Event, keyboard::Keycode, pixels::Color, render::RendererBuilder};

use sdl2_image::{self as image, LoadTexture, INIT_PNG};

use std::{env::current_dir, thread, time::Duration};

fn main() {
    println!("Hello, world!");

    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video
        .window("Rust SDL2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = RendererBuilder::new(window)
        .present_vsync()
        .build()
        .unwrap();

    let _image_context = image::init(INIT_PNG).unwrap();

    let mut app_dir = current_dir().unwrap();
    app_dir.push("assets/sprite_sheet.png");
    let sprite_sheet = canvas.load_texture(&app_dir).unwrap();

    canvas.set_draw_color(Color::RGB(0, 250, 250));
    canvas.clear();
    canvas.present();

    let mut events = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'run_loop: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        canvas.copy(&sprite_sheet, None, None).unwrap();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'run_loop,
                _ => {}
            }
        }

        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
