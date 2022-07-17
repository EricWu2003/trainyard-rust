pub mod shrinking_circle;
pub mod shrinking_plus;
pub mod splitter_particle;
pub mod painter_particle;

use crate::sprites::GameSprites;
use sdl2::render::WindowCanvas;

pub type ParticleList = Vec<Box<dyn Particle>>;

pub trait Particle {
    fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) ->  Result<(), String> ;
    fn pass_one_frame(&mut self) -> ();
    fn still_exists(&self) -> bool;
}