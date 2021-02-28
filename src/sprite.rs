use sdl2::{
    image::LoadTexture,
    rect::Rect,
    render::{Texture, TextureQuery},
};

use std::{convert::TryInto, path::Path, rc::Rc};

use crate::{
    drawable::{Drawable, Renderer},
    utils,
};

/// Load an image as a spritesheet with a grid that starts on 0,0
pub fn load_grid_sprite_sheet<Loader: LoadTexture, P: AsRef<Path>>(
    loader: &Loader,
    filename: P,
    grid_size: u32,
) -> Result<Vec<Sprite<'_>>, String> {
    let texture = Rc::new(loader.load_texture(filename.as_ref())?);

    let (width, height) = texture_size(&texture);

    let mut sprites = vec![];

    for y in (0..height).filter(|val| val % grid_size == 0) {
        for x in (0..width).filter(|val| val % grid_size == 0) {
            let (x, y): (i32, i32) = utils::map_tuple((x, y), |val| val.try_into().unwrap());
            let rect: Rect = (x, y, grid_size, grid_size).into();
            sprites.push(Sprite::from_sheet(texture.clone(), rect)?);
        }
    }

    Ok(sprites)
}

/// A sprite is a square mask on another texture
#[derive(Clone)]
pub struct Sprite<'a> {
    sheet: Rc<Texture<'a>>,
    mask: Rect,
}

impl<'a> Sprite<'a> {
    fn from_sheet(sheet: Rc<Texture<'a>>, rect: Rect) -> Result<Self, String> {
        let (sheet_width, sheet_height) =
            utils::map_tuple(texture_size(&sheet), |val| val.try_into().unwrap());

        if rect.right() <= 0 {
            Err("Mask is too far left".to_string())
        } else if rect.bottom() <= 0 {
            Err("Mask is too far up".to_string())
        } else if rect.left() > sheet_width {
            Err("Mask is too far right".to_string())
        } else if rect.top() > sheet_height {
            Err("Mask is too far down".to_string())
        } else {
            Ok(Sprite { sheet, mask: rect })
        }
    }
}

impl Drawable for Sprite<'_> {
    fn draw_on(&self, dest: &mut Renderer, target_area: Rect) -> Result<(), String> {
        dest.copy(&self.sheet, Some(self.mask), Some(target_area))
    }
}

fn texture_size(texture: &Texture) -> (u32, u32) {
    let TextureQuery { width, height, .. } = texture.query();
    (width, height)
}
