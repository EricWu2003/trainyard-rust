use crate::particle::Particle;
use sdl2::render::WindowCanvas;
use crate::GameSprites;
use crate::utils::centered_rect;
use crate::color::Color;
use rand::Rng;

pub static INITIAL_TTL: i32 = 130;
pub static RANGE: i32 = 20;

pub struct Fire {
    fires: [FireParticle;4],
    ttl: i32,
}

impl Fire {
    pub fn new(x: i32, y:i32, color: Color) -> Fire {
        Fire {
            fires: [
                FireParticle::new(x, y, color, true),
                FireParticle::new(x, y, color, false),
                FireParticle::new(x, y, color, false),
                FireParticle::new(x, y, color, false),
            ],
            ttl: INITIAL_TTL,
        }
    }
}


impl Particle for Fire {
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        for fire in &self.fires {
            fire.render(canvas, gs)?;
        }
        Ok(())
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
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    color: Color,
    ttl: i32,
    is_big: bool,
}

impl FireParticle {
    pub fn new(x: i32, y:i32, color: Color, is_big: bool) -> FireParticle {
        let angle: f64 = rand::thread_rng().gen_range(0.0..6.283185);
        let v_magnitude = 0.15;
        
        let range = if is_big {RANGE/2} else {RANGE};

        let x = x + rand::thread_rng().gen_range(-range..range);
        let y = y + rand::thread_rng().gen_range(-range..range);

        FireParticle {
            x: x as f64,
            y: y as f64,
            dx: v_magnitude * angle.sin(),
            dy: v_magnitude * angle.cos(),
            color,
            ttl: INITIAL_TTL,
            is_big,
        }
    }
}


impl Particle for FireParticle {
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        let source_rect = if self.is_big {
            gs.fire
        } else {
            gs.fire_small
        };

        let rect = centered_rect(self.x as i32, self.y as i32, source_rect.width(), source_rect.height());

        gs.set_color(self.color);
        gs.set_alpha((self.ttl * 255 / INITIAL_TTL) as u8);
        canvas.copy(&gs.atlas_color, source_rect, rect)?;
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