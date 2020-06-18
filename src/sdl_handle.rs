use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Renderer, Texture},
    EventPump, Sdl,
};

use sdl2_image::{self as image, LoadTexture, Sdl2ImageContext};

use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use crate::drawable::Drawable;
use crate::utils;

pub struct SDLHandle {
    _image_context: Sdl2ImageContext,
    sdl_context: Sdl,
    canvas: Renderer<'static>,
    app_directory: PathBuf,
}

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
            .renderer()
            .present_vsync()
            .build()
            .map_err(|e| format!("Error building canvas: {}", e))?;

        canvas.set_draw_color(Color::RGB(0, 250, 250));
        if let Err(e) = canvas.set_logical_size(width, height) {
            eprintln!("{}", e);
        }
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
        self.canvas.clear();
    }
}

impl LoadTexture for SDLHandle {
    fn load_texture(&self, filename: &Path) -> Result<Texture, String> {
        self.canvas.load_texture(filename)
    }
}
