use crate::particle::Particle;
use sdl2::render::WindowCanvas;
use crate::GameSprites;
use crate::utils::centered_rect;
use crate::color::Color;
use rand::Rng;

pub static INITIAL_TTL: i32 = 50;
pub static RANGE: i32 = 30;

pub struct Sparkle {
    stars: [Star;3],
    ttl: i32,
}

impl Sparkle {
    pub fn new(x: i32, y:i32, color: Color) -> Sparkle {
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
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        for star in &self.stars {
            star.render(canvas, gs)?;
        }
        Ok(())
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
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    color: Color,
    ttl: i32,
}

impl Star {
    pub fn new(x: i32, y:i32, color: Color) -> Star {
        let angle: f64 = rand::thread_rng().gen_range(0.0..6.283185);
        let v_magnitude = 0.15;
        let x = x + rand::thread_rng().gen_range(-RANGE..RANGE);
        let y = y + rand::thread_rng().gen_range(-RANGE..RANGE);

        Star {
            x: x as f64,
            y: y as f64,
            dx: v_magnitude * angle.sin(),
            dy: v_magnitude * angle.cos(),
            color,
            ttl: INITIAL_TTL,
        }
    }
}


impl Particle for Star {
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        println!("star rendering");
        println!("{}, {}", self.x, self.y);

        let rect = centered_rect(self.x as i32, self.y as i32, gs.star.width(), gs.star.height());

        gs.set_color(self.color);
        gs.set_alpha((self.ttl * 255 / INITIAL_TTL) as u8);
        canvas.copy(&gs.atlas_color, gs.star_bright, rect)?;
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