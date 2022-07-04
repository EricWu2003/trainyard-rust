use crate::Connection;
use crate::edge::Edge;
use crate::tile::Tile;
use crate::tile::tracktile::Tracktile;

pub const NUM_ROWS: usize = 7;
pub const NUM_COLS: usize = 7;

pub struct Yard {
    tiles: Vec<Vec<Tile>>,
    h_edges: Vec<Vec<Edge>>,
    v_edges: Vec<Vec<Edge>>,
}

impl Yard {
    pub fn new() -> Yard {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for _ in 0..NUM_ROWS {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..NUM_COLS {
                row.push(Tile::Tracktile(Tracktile::new(None, None)));
            }
            tiles.push(row);
        }
        
        let mut h_edges: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..(NUM_ROWS+1) {
            let mut row: Vec<Edge> = Vec::new();
            for _ in 0..NUM_COLS {
                row.push(Edge::new());
            }
            h_edges.push(row);
        }

        let mut v_edges: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..NUM_ROWS {
            let mut row: Vec<Edge> = Vec::new();
            for _ in 0..(NUM_COLS+1) {
                row.push(Edge::new());
            }
            v_edges.push(row);
        }
        tiles[0][0] .add_connection(Connection{dir1:1, dir2:2});



        Yard {
            tiles, h_edges, v_edges
        }

    }
}