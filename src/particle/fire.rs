use macroquad::prelude::*;
use crate::particle::Particle;
use crate::GameSprites;
use crate::color::Color;
use macroquad::rand::gen_range;

pub static INITIAL_TTL: i32 = 130;
pub static RANGE: f32 = 20.;

pub struct Fire {
    fires: [FireParticle;4],
    ttl: i32,
}

impl Fire {
    pub fn new(x: f32, y:f32, color: Color, scale: f32) -> Fire {
        Fire {
            // We'll make one big fire and three small fires.
            fires: [
                FireParticle::new(x, y, color, true, scale),
                FireParticle::new(x, y, color, false, scale),
                FireParticle::new(x, y, color, false, scale),
                FireParticle::new(x, y, color, false, scale),
            ],
            ttl: INITIAL_TTL,
        }
    }
}


impl Particle for Fire {
    fn render(&self, gs: &GameSprites) {
        for fire in &self.fires {
            fire.render(gs);
        }
    }
    fn pass_one_frame(&mut self) {
        for fire in &mut self.fires {
            fire.pass_one_frame();
        }
        self.ttl -= 1;
    }
    fn still_exists(&self) -> bool {
        self.ttl > 0
    }
}

pub struct FireParticle {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    color: Color,
    ttl: i32,
    is_big: bool,
    scale: f32
}

impl FireParticle {
    pub fn new(x: f32, y:f32, color: Color, is_big: bool, scale: f32) -> FireParticle {


        let angle: f32 = gen_range(0.0, 6.283185);
        let v_magnitude = 0.15 * scale;

        // For a nice fire effect,
        // we want the big fire particle to be closer to the cluster's center,
        // and smaller particles can be further away.
        let range = if is_big {RANGE * scale / 2.} else {RANGE * scale};

        let x = x + gen_range(-range, range);
        let y = y + gen_range(-range, range);

        FireParticle {
            x: x,
            y: y,
            dx: v_magnitude * angle.sin(),
            dy: v_magnitude * angle.cos(),
            color,
            ttl: INITIAL_TTL,
            is_big,
            scale,
        }
    }
}


impl Particle for FireParticle {
    fn render(&self, gs: &GameSprites) {
        let texture_to_draw = if self.is_big {
            gs.fire
        } else {
            gs.fire_small
        };

        let (texture_width, texture_height) = (texture_to_draw.width() * self.scale/2., texture_to_draw.height() * self.scale/2.);

        
        let mut color = self.color.get_color();
        color.a = self.ttl as f32 / INITIAL_TTL as f32;
        draw_texture_ex(
            texture_to_draw,
            self.x - texture_width/2.,
            self.y - texture_height/2.,
            color,
            DrawTextureParams { 
                dest_size: Some(Vec2::new(texture_width, texture_height)),
                source: None,
                rotation: 0.,
                flip_x: false,
                flip_y: false,
                pivot: None
            }
        )
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