use macroquad::prelude::*;
use crate::color::Color;
use crate::particle::Particle;
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
    fn render(&self, gs: &GameSprites) {
        println!("TODO: render printer particle");
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