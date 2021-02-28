use sdl2::{
    image::{self, Sdl2ImageContext},
    pixels::Color,
    rect::{Point, Rect},
    render::TextureCreator,
    video::WindowContext,
    EventPump, Sdl,
};

use std::{env::current_dir, path::PathBuf};

use crate::drawable::{Drawable, Renderer};
use crate::utils;

pub struct SDLHandle {
    _image_context: Sdl2ImageContext,
    sdl_context: Sdl,
    canvas: Renderer,
    app_directory: PathBuf,
}

const CLEAR_COLOR: Color = Color::RGB(0, 250, 250);

impl SDLHandle {
    pub fn init(
        window_title: &str,
        (width, height): (u32, u32),
        image_init: image::InitFlag,
    ) -> Result<SDLHandle, String> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;

        let window = video
            .window(window_title, width, height)
            .position_centered()
            .build()
            .map_err(|e| format!("Error building window: {}", e))?;

        let mut canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| format!("Error building canvas: {}", e))?;

        if let Err(e) = canvas.set_logical_size(width, height) {
            eprintln!("{}", e);
        }

        canvas.set_draw_color(CLEAR_COLOR);
        canvas.clear();
        canvas.present();

        let _image_context = image::init(image_init)?;

        let app_directory = current_dir().map_err(|e| format!("{}", e))?;

        Ok(Self {
            _image_context,
            sdl_context,
            canvas,
            app_directory,
        })
    }

    pub fn event_pump(&self) -> Result<EventPump, String> {
        self.sdl_context.event_pump()
    }

    pub fn draw_size(&self) -> (u32, u32) {
        self.canvas.logical_size()
    }

    pub fn center_of_draw(&self) -> (u32, u32) {
        utils::map_tuple(self.draw_size(), |val| val / 2)
    }

    pub fn asset_path(&self, asset_name: &str) -> PathBuf {
        let mut path = self.app_directory.clone();
        path.push("assets/");
        path.push(asset_name);
        path
    }

    pub fn draw_at<D: Drawable>(&mut self, pos: Point, thing: &D) -> Result<(), String> {
        thing.draw_at(&mut self.canvas, pos)
    }

    pub fn draw_on<D: Drawable>(&mut self, area: Rect, thing: &D) -> Result<(), String> {
        thing.draw_on(&mut self.canvas, area)
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(CLEAR_COLOR);
        self.canvas.clear();
    }

    pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }
}
