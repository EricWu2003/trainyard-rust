use macroquad::prelude::*;
use crate::particle::Particle;
use crate::color::Color;
use crate::GameSprites;

pub static INITIAL_TTL: i32 = 20;

pub struct ShrinkingPlus {
    bounding_rect: Rect,
    color: Color,
    ttl: i32,
}

impl ShrinkingPlus {
    pub fn new(rect: Rect, color: Color) -> ShrinkingPlus{
        ShrinkingPlus {bounding_rect: rect, color, ttl: INITIAL_TTL}
    }
}

impl Particle for ShrinkingPlus {
    fn render(&self, gs: &GameSprites) {
        let scale:f32 = self.ttl as f32 / INITIAL_TTL as f32;

        let new_x = self.bounding_rect.x + (0.5 * (1.0-scale) * self.bounding_rect.w );
        let new_y = self.bounding_rect.y + (0.5 * (1.0-scale) * self.bounding_rect.h );
        let new_h: f32 = self.bounding_rect.h * scale;
        let new_w: f32 = self.bounding_rect.w * scale;

        draw_texture_ex(gs.plus_sign, new_x, new_y, self.color.get_color(), 
            DrawTextureParams { 
                dest_size: Some(Vec2::new(new_w, new_h)),
                source: None,
                rotation: 0.0,
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