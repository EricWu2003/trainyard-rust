use macroquad::prelude::*;
use crate::particle::Particle;
use crate::GameSprites;
use std::f32::consts::PI;

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
    fn render(&self, gs: &GameSprites) {
        let source_size = gs.splitter_animation.height();
        let source_rect = Rect::new((self.ttl * NUM_FRAMES / INITIAL_TTL) as f32 * source_size, 0., source_size, source_size);

        let dest_size = Vec2::new(self.bounding_rect.w, self.bounding_rect.h);

        draw_texture_ex(
            gs.splitter_animation,
            self.bounding_rect.x,
            self.bounding_rect.y,
            WHITE,
            DrawTextureParams { 
                dest_size: Some(dest_size),
                source: Some(source_rect),
                rotation:  PI/2. * self.dir as f32,
                flip_x: false,
                flip_y: false,
                pivot: None
            }
        )

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