use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::particle::Particle;
use crate::GameSprites;
use crate::color::Color;

pub static INITIAL_TTL: i32 = 50;
pub static RANGE: f32 = 30.;

pub struct Sparkle {
    stars: [Star;3],
    ttl: i32,
}

impl Sparkle {
    pub fn new(x: f32, y:f32, color: Color, scale: f32) -> Sparkle {
        Sparkle {
            stars: [
                Star::new(x, y, color, scale),
                Star::new(x, y, color, scale),
                Star::new(x, y, color, scale),
            ],
            ttl: INITIAL_TTL,
        }
    }
}


impl Particle for Sparkle {
    fn render(&self, gs: &GameSprites) {
        for star in &self.stars {
            star.render(gs);
        }
    }
    fn pass_one_frame(&mut self) {
        for star in &mut self.stars {
            star.pass_one_frame();
        }
        self.ttl -= 1;
    }
    fn still_exists(&self) -> bool {
        self.ttl > 0
    }
}

pub struct Star {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    color: Color,
    ttl: i32,
    scale: f32,
}

impl Star {
    pub fn new(x: f32, y:f32, color: Color, scale: f32) -> Star {
        let angle: f32 = gen_range(0.0, 6.283185);
        let v_magnitude = 0.15 * scale;
        let range = RANGE * scale;
        let x = x + gen_range(-range, range);
        let y = y + gen_range(-range, range);

        Star {
            x,
            y,
            dx: v_magnitude * angle.sin(),
            dy: v_magnitude * angle.cos(),
            color,
            ttl: INITIAL_TTL,
            scale: scale,
        }
    }
}


impl Particle for Star {
    fn render(&self, gs: &GameSprites) {
        let (w, h) = (self.scale * gs.star_bright.width(), self.scale * gs.star_bright.height());


        let mut color = self.color.get_color();
        color.a = self.ttl as f32 / INITIAL_TTL as f32;
        draw_texture_ex(
            gs.star_bright,
            self.x - w/2., 
            self.y - w/2., 
            color,
            DrawTextureParams {
                dest_size: Some(Vec2::new(w, h)),
                source: None,
                rotation: 0.,
                flip_x: false,
                flip_y: false,
                pivot: None,
            }
        );
    }
    fn pass_one_frame(&mut self) {
        self.ttl -= 1;
        self.x += self.dx;
        self.y += self.dy;
        if self.ttl < 0 {
            panic!("we should have removed this particle once still_exists returns false!")
        }
    }
    fn still_exists(&self) -> bool {
        self.ttl > 0
    }
}