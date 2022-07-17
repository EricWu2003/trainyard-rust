use crate::color::Color;
use crate::particle::Particle;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::GameSprites;

pub static INITIAL_TTL: i32 = 20;
pub static NUM_FRAMES: i32 = 10;

pub struct PainterParticle {
    bounding_rect: Rect,
    ttl: i32,
    color: Color,
}

impl PainterParticle {
    pub fn new(rect: Rect, color: Color) -> PainterParticle{
        PainterParticle {bounding_rect: rect, ttl: INITIAL_TTL, color}
    }
}

impl Particle for PainterParticle {
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        let rect = Rect::new((self.ttl * NUM_FRAMES / INITIAL_TTL) * 96, 496, 96, 96);

        gs.set_color(self.color);
        canvas.copy(&gs.atlas_color, rect, self.bounding_rect)?;
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