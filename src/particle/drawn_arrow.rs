use macroquad::prelude::*;
use crate::particle::Particle;
use crate::GameSprites;
use std::f32::consts::PI;

pub static INITIAL_TTL: i32 = 100;

pub struct DrawnArrow {
    x: f32,
    y: f32,
    dir: u8,
    ttl: i32,
    scale: f32
}

impl DrawnArrow {
    pub fn new(x:f32, y:f32, dir:u8, scale:f32) -> DrawnArrow{
        DrawnArrow {x, y, dir, ttl: INITIAL_TTL, scale}
    }
}

impl Particle for DrawnArrow {
    fn render(&self, gs: &GameSprites) {
        let scale = self.scale;
        let x = self.x - gs.draw_track_arrow.width() * scale /2.;
        let y = self.y - gs.draw_track_arrow.height() * scale /2.;

        let dest_size = Some(Vec2::new(gs.draw_track_arrow.width() * scale, gs.draw_track_arrow.height() * scale));

        draw_texture_ex(
            gs.draw_track_arrow,
            x,
            y,
            WHITE,
            DrawTextureParams { 
                dest_size, 
                source: None, 
                rotation: self.dir as f32 * PI/2., 
                flip_x: false, 
                flip_y: false, 
                pivot: None,
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