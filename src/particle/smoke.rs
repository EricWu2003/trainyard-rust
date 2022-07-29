use macroquad::prelude::*;
use crate::particle::Particle;
use crate::GameSprites;
use crate::utils::centered_rect;
use crate::color::Color;
use macroquad::rand::gen_range;

pub static INITIAL_TTL: i32 = 170;

pub struct Smoke {
    smokes: [SmokeParticle;3],
    ttl: i32,
}

impl Smoke {
    pub fn new(x: f32, y:f32, color: Color) -> Smoke {
        Smoke {
            smokes: [
                SmokeParticle::new(x, y, color),
                SmokeParticle::new(x, y, color),
                SmokeParticle::new(x, y, color),
            ],
            ttl: INITIAL_TTL,
        }
    }
}


impl Particle for Smoke {
    fn render(&self, gs: &GameSprites) {
        for smoke in &self.smokes {
            smoke.render(gs);
        }
    }
    fn pass_one_frame(&mut self) {
        for smoke in &mut self.smokes {
            smoke.pass_one_frame();
        }
        self.ttl -= 1;
    }
    fn still_exists(&self) -> bool {
        self.ttl > 0
    }
}

pub struct SmokeParticle {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    color: Color,
    ttl: i32,
}

impl SmokeParticle {
    pub fn new(x: f32, y:f32, color: Color) -> SmokeParticle {
        let angle: f32 = gen_range(0.0, 6.283185);
        let v_magnitude = 0.15;
        SmokeParticle {
            x: x,
            y: y,
            dx: v_magnitude * angle.sin(),
            dy: v_magnitude * angle.cos(),
            color,
            ttl: INITIAL_TTL,
        }
    }
}


impl Particle for SmokeParticle {
    fn render(&self, gs: &GameSprites) {

        // gs.set_color(self.color);
        // gs.set_alpha((self.ttl * 255 / INITIAL_TTL) as u8);
        draw_texture(gs.smoke, self.x, self.y, WHITE);
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