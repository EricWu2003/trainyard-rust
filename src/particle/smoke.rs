use crate::particle::Particle;
use sdl2::render::WindowCanvas;
use crate::GameSprites;
use crate::utils::centered_rect;
use crate::color::Color;
use rand::Rng;

pub static INITIAL_TTL: i32 = 170;

pub struct Smoke {
    smokes: [SmokeParticle;3],
    ttl: i32,
}

impl Smoke {
    pub fn new(x: i32, y:i32, color: Color) -> Smoke {
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
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        for smoke in &self.smokes {
            smoke.render(canvas, gs)?;
        }
        Ok(())
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
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    color: Color,
    ttl: i32,
}

impl SmokeParticle {
    pub fn new(x: i32, y:i32, color: Color) -> SmokeParticle {
        let angle: f64 = rand::thread_rng().gen_range(0.0..6.283185);
        let v_magnitude = 0.15;
        SmokeParticle {
            x: x as f64,
            y: y as f64,
            dx: v_magnitude * angle.sin(),
            dy: v_magnitude * angle.cos(),
            color,
            ttl: INITIAL_TTL,
        }
    }
}


impl Particle for SmokeParticle {
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        let rect = centered_rect(self.x as i32, self.y as i32, gs.smoke.width(), gs.smoke.height());

        gs.set_color(self.color);
        gs.set_alpha((self.ttl * 255 / INITIAL_TTL) as u8);
        canvas.copy(&gs.atlas_color, gs.smoke, rect)?;
        gs.set_alpha(255);
        Ok(())
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