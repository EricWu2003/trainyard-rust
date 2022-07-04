use crate::edge::Edge;
use crate::tile::Tile;

pub const NUM_ROWS: u8 = 7;
pub const NUM_COLS: u8 = 7;

pub struct Yard {
    tiles: Vec<Vec<Tile>>,
    borders: Edge,
}
