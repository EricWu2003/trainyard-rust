use crate::particle::Particle;
use crate::color::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::GameSprites;

pub static INITIAL_TTL: i32 = 30;

pub struct ShrinkingCircle {
    bounding_rect: Rect,
    color: Color,
    ttl: i32,
}

impl ShrinkingCircle {
    pub fn new(rect: Rect, color: Color) -> ShrinkingCircle{
        ShrinkingCircle {bounding_rect: rect, color, ttl: INITIAL_TTL}
    }
}

impl Particle for ShrinkingCircle {
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) {
        // TODO:
    }
    fn pass_one_frame(&mut self) {
        self.ttl -= 1;
        if self.ttl < 0 {
            panic!("we should have removed this particle once still_exists returns false!")
        }
    }
    fn still_exists(&self) -> bool {
        self.ttl > 0
    }
}