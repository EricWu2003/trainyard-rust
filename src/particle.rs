pub mod shrinking_circle;
pub mod shrinking_plus;
pub mod splitter_particle;
pub mod painter_particle;
pub mod drawn_arrow;
pub mod smoke;
pub mod sparkle;
pub mod fire;

use crate::sprites::GameSprites;

pub type ParticleList = Vec<Box<dyn Particle>>;

pub trait Particle {
    fn render(&self, gs: &GameSprites);
    fn pass_one_frame(&mut self);
    fn still_exists(&self) -> bool;
}