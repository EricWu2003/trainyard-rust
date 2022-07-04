use crate::Connection;
use crate::edge::Edge;
use crate::tile::Tile;
use crate::tile::tracktile::Tracktile;
use crate::tile::BorderState;
use crate::color::Color;
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
        v_edges[0][1].train_to_a = Some(Color::Blue);
        h_edges[1][0].train_to_a = Some(Color::Green);

        // END OF DEBUG CODE

        Yard { tiles, h_edges, v_edges }

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

    pub fn process_tick(&mut self) {
        // detect crashes on boundaries (i.e. if a train is about to crash by going 
        // too far up where there is no tile left to catch it)
        for c in 0..NUM_COLS {
            if let Some(_train) = self.h_edges[0][c].train_to_a {
                panic!("train crash!");
            }
            if let Some(_train) = self.h_edges[NUM_ROWS][c].train_to_b {
                panic!("train crash!");
            }
        }
        for r in 0..NUM_ROWS {
            if let Some(_train) = self.v_edges[r][0].train_to_a {
                panic!("train crash!");
            }
            if let Some(_train) = self.v_edges[r][NUM_COLS].train_to_b {
                panic!("train crash!");
            }
        }


        // first all tiles pull in trains from the edges. A crash occurs if there is a
        // train entering a tile but the tile does not pull it in.
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                let border_state: BorderState = [
                    self.h_edges[r][c].train_to_b,
                    self.v_edges[r][c+1].train_to_a,
                    self.h_edges[r+1][c].train_to_a,
                    self.v_edges[r][c].train_to_b,
                ];
                let res = self.tiles[r][c].accept_trains(border_state);
                if !res {
                    panic!("train crash!");
                }
            }
        }

        // then process tick in all tiles
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                self.tiles[r][c].process_tick();
            }
        }

        // then dispatch all trains and store them in edges.
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                [
                    self.h_edges[r][c].train_to_a,
                    self.v_edges[r][c+1].train_to_b,
                    self.h_edges[r+1][c].train_to_b,
                    self.v_edges[r][c].train_to_a,
                ] = self.tiles[r][c].dispatch_trains();
            }
        }
        // mix edges
        for r in 0..(NUM_ROWS+1) {
            for c in 0..NUM_COLS {
                self.h_edges[r][c].interact_trains();
            }
        }
        for r in 0..NUM_ROWS {
            for c in 0..(NUM_COLS+1) {
                self.v_edges[r][c].interact_trains();
            }
        }


       
        
    }
}