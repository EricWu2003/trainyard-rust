use crate::particle::Particle;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::GameSprites;

pub static INITIAL_TTL: i32 = 10;
pub static NUM_FRAMES: i32 = 5;

pub struct SplitterParticle {
    bounding_rect: Rect,
    ttl: i32,
    dir: u8,
}

impl SplitterParticle {
    pub fn new(rect: Rect, dir: u8) -> SplitterParticle{
        SplitterParticle {bounding_rect: rect, ttl: INITIAL_TTL, dir}
    }
}

impl Particle for SplitterParticle {
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        let rect = Rect::new((self.ttl * NUM_FRAMES / INITIAL_TTL) * 96, 400, 96, 96);

        canvas.copy_ex(&gs.atlas, rect, self.bounding_rect,
            90.0 * self.dir as f64, None, false, false)?;
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