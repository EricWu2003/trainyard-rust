use crate::Connection;
use crate::edge::Edge;
use crate::tile::Tile;
use crate::tile::tracktile::Tracktile;
use std::io::Write;

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

        // DEBUG CODE HERE
        tiles[0][0] .add_connection(Connection{dir1:1, dir2:2});
        tiles[0][0] .add_connection(Connection{dir1:0, dir2:2});


        // END OF DEBUG CODE

        Yard {
            tiles, h_edges, v_edges
        }

    }

    pub fn display(&self) {
        for r in 0..NUM_ROWS {
            print!(" ");
            for c in 0..NUM_COLS {
                print!("{}", self.h_edges[r][c].get_char().to_string() + " ");
            }
            println!("");
            for c in 0..NUM_COLS {
                print!("{}", self.v_edges[r][c].get_char().to_string() + &self.tiles[r][c].get_char().to_string());
            }
            println!("{}", self.v_edges[r][NUM_ROWS].get_char());
        }
        
        print!(" ");
        for c in 0..NUM_COLS {
            print!("{}", self.h_edges[NUM_ROWS][c].get_char().to_string() + " ");
        }
        println!("");
        
        std::io::stdout().flush().unwrap();
    }
}