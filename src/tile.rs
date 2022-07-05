pub mod tracktile;
pub mod trainsink;
pub mod trainsource;
use crate::color::Color;
use crate::connection::Connection;
use crate::tile::tracktile::Tracktile;
use crate::tile::trainsink::Trainsink;
use crate::tile::trainsource::Trainsource;

pub type BorderState = [Option<Color>; 4];

pub enum Tile {
    Tracktile(Tracktile),
    Trainsource(Trainsource),
    Trainsink(Trainsink),
}

impl Tile {
    pub fn accept_trains(&mut self, trains: BorderState) -> bool {
        match self {
            Tile::Tracktile(tracktile) => tracktile.accept_trains(trains),
            Tile::Trainsource(trainsource) => trainsource.accept_trains(trains),
            Tile::Trainsink(trainsink) => trainsink.accept_trains(trains),
        }
    }
    pub fn dispatch_trains(&mut self) -> BorderState {
        match self {
            Tile::Tracktile(tracktile) => tracktile.dispatch_trains(),
            Tile::Trainsource(trainsource) => trainsource.dispatch_trains(),
            Tile::Trainsink(trainsink) => trainsink.dispatch_trains(),
        }
    }
    pub fn process_tick(&mut self) {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.interact_trains();
            }
            Tile::Trainsource(_) => {}
            Tile::Trainsink(_) => {}
        }
    }
    pub fn add_connection(&mut self, conn: Connection) {
        match self {
            Tile::Tracktile(tracktile) => {
                tracktile.add_connection(conn);
            }
            Tile::Trainsource(_) => {}
            Tile::Trainsink(_) => {}
        }
    }
    pub fn get_char(&self) -> char {
        match self {
            Tile::Tracktile(tracktile) => tracktile.connection_type().get_char(),
            Tile::Trainsource(_) => 'S',
            Tile::Trainsink(_) => 'S',
        }
    }
}
