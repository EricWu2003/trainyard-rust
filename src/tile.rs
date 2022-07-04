pub mod tracktile;
use crate::color::Color;
use crate::{Tracktile, Connection};

pub type BorderState = [Option<Color>; 4];

pub enum Tile {
    Tracktile(Tracktile),
}

impl Tile {
    pub fn accept_trains(&mut self, trains: BorderState) -> bool {
        match self {
            Tile::Tracktile(tracktile) => tracktile.accept_trains(trains),
        }
    }
    pub fn dispatch_trains(&mut self) -> BorderState {
        match self {
            Tile::Tracktile(tracktile) => tracktile.dispatch_trains(),
        }
    }
    pub fn process_tick(&mut self) {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.interact_trains();
            }
        }
    }
    pub fn add_connection(&mut self, conn: Connection) {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.add_connection(conn);
            }
        }
    }
}
