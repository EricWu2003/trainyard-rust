use macroquad::prelude::*;
use macroquad::rand::gen_range;
use crate::particle::Particle;
use crate::GameSprites;
use crate::utils::centered_rect;
use crate::color::Color;

pub static INITIAL_TTL: i32 = 50;
pub static RANGE: f32 = 30.;

pub struct Sparkle {
    stars: [Star;3],
    ttl: i32,
}

impl Sparkle {
    pub fn new(x: f32, y:f32, color: Color) -> Sparkle {
        Sparkle {
            stars: [
                Star::new(x, y, color),
                Star::new(x, y, color),
                Star::new(x, y, color),
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
}

impl Star {
    pub fn new(x: f32, y:f32, color: Color) -> Star {
        let angle: f32 = gen_range(0.0, 6.283185);
        let v_magnitude = 0.15;
        let x = x + gen_range(-RANGE, RANGE);
        let y = y + gen_range(-RANGE, RANGE);

        Star {
            x,
            y,
            dx: v_magnitude * angle.sin(),
            dy: v_magnitude * angle.cos(),
            color,
            ttl: INITIAL_TTL,
        }
    }
}


impl Particle for Star {
    fn render(&self, gs: &GameSprites) {

        // gs.set_color(self.color);
        // gs.set_alpha((self.ttl * 255 / INITIAL_TTL) as u8);
        // canvas.copy(&gs.atlas_color, gs.star_bright, rect)?;
        draw_texture(gs.star_bright, self.x, self.y, WHITE);
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