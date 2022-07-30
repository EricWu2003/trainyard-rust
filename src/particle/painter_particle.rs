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
        let source_size = gs.painter_brush_animation.height();
        let source_rect = Rect::new((self.ttl * NUM_FRAMES / INITIAL_TTL) as f32 * source_size, 0., source_size, source_size);

        let dest_size = Vec2::new(self.bounding_rect.w, self.bounding_rect.h);

        draw_texture_ex(
            gs.painter_brush_animation,
            self.bounding_rect.x,
            self.bounding_rect.y,
            self.color.get_color(),
            DrawTextureParams { 
                dest_size: Some(dest_size),
                source: Some(source_rect),
                rotation: 0.,
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