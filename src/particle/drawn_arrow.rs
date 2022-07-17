use crate::particle::Particle;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::GameSprites;

pub static INITIAL_TTL: i32 = 100;

pub struct DrawnArrow {
    x: i32,
    y: i32,
    dir: u8,
    ttl: i32,
}

impl DrawnArrow {
    pub fn new(x:i32, y:i32, dir:u8) -> DrawnArrow{
        DrawnArrow {x, y, dir, ttl: INITIAL_TTL}
    }
}

impl Particle for DrawnArrow {
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        let x = self.x - gs.draw_track_arrow.width() as i32/2;
        let y = self.y - gs.draw_track_arrow.height() as i32/2;
        let w = gs.draw_track_arrow.width();
        let h = gs.draw_track_arrow.height();

        canvas.copy_ex(&gs.atlas, gs.draw_track_arrow, Rect::new(x, y, w, h),
            self.dir as f64 * 90.0, None, false, false)?;
        Ok(())
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