use sdl2::{
    rect::{Point, Rect},
    render::Renderer,
};

pub trait Drawable {
    fn draw_on(&self, dest: &mut Renderer, area: Rect) -> Result<(), String> {
        eprintln!("Drawing on center");
        self.draw_at(dest, area.center())
    }

    fn draw_at(&self, _dest: &mut Renderer, _pos: Point) -> Result<(), String> {
        unreachable!("Unimplemented draw_on");
    }
}
