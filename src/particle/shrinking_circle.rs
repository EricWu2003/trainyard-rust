use crate::particle::Particle;
use crate::color::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::GameSprites;

pub static INITIAL_TTL: i32 = 20;

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
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        let scale:f64 = self.ttl as f64 / INITIAL_TTL as f64;

        let new_x = self.bounding_rect.x() + (0.5 * (1.0-scale) * self.bounding_rect.width() as f64) as i32;
        let new_y = self.bounding_rect.y() + (0.5 * (1.0-scale) * self.bounding_rect.height() as f64) as i32;
        let new_h: u32 = self.bounding_rect.height() * self.ttl as u32/INITIAL_TTL as u32;
        let new_w: u32 = self.bounding_rect.width() * self.ttl as u32/INITIAL_TTL as u32;

        gs.set_color(self.color);
        canvas.copy(&gs.atlas_color, gs.circle, Rect::new(new_x, new_y, new_w, new_h))?;
        Ok(())
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