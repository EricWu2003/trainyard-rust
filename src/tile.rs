pub mod tracktile;
use crate::color::Color;



pub type BorderState = [Option<Color>; 4];

pub trait Tile {
    fn accept_trains(&mut self, trains: BorderState) -> bool;
    fn dispatch_trains(&mut self) -> BorderState;
}

