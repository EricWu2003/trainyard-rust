use crate::color::Color;
use crate::connection::Connection;
use crate::edge::Edge;
use crate::sprites::GameSprites;
use crate::tile::tracktile::ConnectionType;
use crate::tile::tracktile::Tracktile;
use crate::tile::BorderState;
use crate::tile::Tile;

use std::io::Write;

use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

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
        for _ in 0..(NUM_ROWS + 1) {
            let mut row: Vec<Edge> = Vec::new();
            for _ in 0..NUM_COLS {
                row.push(Edge::new());
            }
            h_edges.push(row);
        }

        let mut v_edges: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..NUM_ROWS {
            let mut row: Vec<Edge> = Vec::new();
            for _ in 0..(NUM_COLS + 1) {
                row.push(Edge::new());
            }
            v_edges.push(row);
        }

        // DEBUG CODE HERE
        tiles[0][0].add_connection(Connection { dir1: 1, dir2: 3 });
        tiles[0][0].add_connection(Connection { dir1: 1, dir2: 2 });
        tiles[0][1].add_connection(Connection { dir1: 1, dir2: 2 });
        tiles[0][1].add_connection(Connection { dir1: 2, dir2: 0 });
        tiles[0][2].add_connection(Connection { dir1: 2, dir2: 1 });
        tiles[0][2].add_connection(Connection { dir1: 1, dir2: 0 });
        // tiles[0][0].add_connection(Connection { dir1: 0, dir2: 2 });
        v_edges[0][1].train_to_a = Some(Color::Blue);
        h_edges[1][0].train_to_a = Some(Color::Green);

        // END OF DEBUG CODE

        Yard {
            tiles,
            h_edges,
            v_edges,
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
                print!(
                    "{}",
                    self.v_edges[r][c].get_char().to_string()
                        + &self.tiles[r][c].get_char().to_string()
                );
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
                    self.v_edges[r][c + 1].train_to_a,
                    self.h_edges[r + 1][c].train_to_a,
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
                    self.v_edges[r][c + 1].train_to_b,
                    self.h_edges[r + 1][c].train_to_b,
                    self.v_edges[r][c].train_to_a,
                ] = self.tiles[r][c].dispatch_trains();
            }
        }
        // mix edges
        for r in 0..(NUM_ROWS + 1) {
            for c in 0..NUM_COLS {
                self.h_edges[r][c].interact_trains();
            }
        }
        for r in 0..NUM_ROWS {
            for c in 0..(NUM_COLS + 1) {
                self.v_edges[r][c].interact_trains();
            }
        }
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        rect: &Rect,
        gs: &GameSprites,
    ) -> Result<(), String> {
        let block_width = (rect.width() / (NUM_COLS as u32)) as i32;
        let block_height = (rect.height() / (NUM_ROWS as u32)) as i32;
        let train_width = (block_width as f64 * (32.0 / 96.0)) as i32;
        let train_height = (block_height as f64 * (57.0 / 96.0)) as i32;
        let x0 = rect.x();
        let y0 = rect.y();

        //render all tracktiles
        for r in 0..NUM_ROWS {
            for c in 0..NUM_COLS {
                let mut texture = &gs.tracktile_blank;
                let mut h_flip = false;
                let mut rot = 0;

                match &self.tiles[r][c] {
                    Tile::Tracktile(tracktile) => match tracktile.connection_type() {
                        ConnectionType::None => {}
                        ConnectionType::B => {
                            texture = &gs.tracktile_b;
                            rot =
                                tracktile.has_connection_up_to_rot(Connection { dir1: 2, dir2: 3 });
                        }
                        ConnectionType::S => {
                            texture = &gs.tracktile_s;
                            rot =
                                tracktile.has_connection_up_to_rot(Connection { dir1: 0, dir2: 2 });
                        }
                        ConnectionType::H => {
                            texture = &gs.tracktile_h;
                            rot = tracktile
                                .has_active_connection_up_to_rot(Connection { dir1: 0, dir2: 2 });
                        }
                        ConnectionType::Z => {
                            texture = &gs.tracktile_z;
                            rot =
                                tracktile.has_connection_up_to_rot(Connection { dir1: 0, dir2: 1 });
                        }
                        ConnectionType::M => {
                            texture = &gs.tracktile_m;
                            if tracktile.has_active_passive_connections_up_to_rot(
                                Connection { dir1: 2, dir2: 3 },
                                Connection { dir1: 1, dir2: 2 },
                            ) != -1
                            {
                                h_flip = false;
                                rot = tracktile.has_active_passive_connections_up_to_rot(
                                    Connection { dir1: 2, dir2: 3 },
                                    Connection { dir1: 1, dir2: 2 },
                                );
                            } else {
                                h_flip = true;
                                rot = tracktile.has_active_passive_connections_up_to_rot(
                                    Connection { dir1: 1, dir2: 2 },
                                    Connection { dir1: 2, dir2: 3 },
                                );
                            }
                        }
                        ConnectionType::J => {
                            if tracktile
                                .has_active_connection_up_to_rot(Connection { dir1: 0, dir2: 1 })
                                != -1
                            {
                                texture = &gs.tracktile_jb;
                            } else {
                                texture = &gs.tracktile_js;
                            }
                            if tracktile.has_connections_up_to_rot(
                                Connection { dir1: 0, dir2: 2 },
                                Connection { dir1: 3, dir2: 2 },
                            ) != -1
                            {
                                h_flip = false;
                                rot = tracktile.has_connections_up_to_rot(
                                    Connection { dir1: 0, dir2: 2 },
                                    Connection { dir1: 3, dir2: 2 },
                                )
                            } else {
                                h_flip = true;
                                rot = tracktile.has_connections_up_to_rot(
                                    Connection { dir1: 0, dir2: 2 },
                                    Connection { dir1: 1, dir2: 2 },
                                )
                            }
                        }
                    },
                }

                canvas.copy_ex(
                    texture,
                    None,
                    Rect::new(
                        x0 + c as i32 * block_width,
                        y0 + r as i32 * block_height,
                        block_width as u32,
                        block_height as u32,
                    ),
                    rot as f64 * 90.0,
                    None,
                    h_flip,
                    false,
                )?;
            }
        }

        //render all trains on borders
        for r in 0..(NUM_ROWS + 1) {
            for c in 0..NUM_COLS {
                let rect = Rect::new(
                    x0 + block_width / 2 + (c as i32) * block_width - (train_width / 2),
                    y0 + r as i32 * block_height - (train_height / 2),
                    train_width as u32,
                    train_height as u32,
                );
                if let Some(_train_going_up) = self.h_edges[r][c].train_to_a {
                    canvas.copy_ex(&gs.train, None, rect, 0.0, None, false, false)?;
                }
                if let Some(_train_going_down) = self.h_edges[r][c].train_to_b {
                    canvas.copy_ex(&gs.train, None, rect, 180.0, None, false, false)?;
                }
            }
        }
        for r in 0..NUM_ROWS {
            for c in 0..(NUM_COLS + 1) {
                let rect = Rect::new(
                    x0 + c as i32 * block_width - (train_width / 2),
                    y0 + block_height / 2 + r as i32 * block_height - (train_height / 2),
                    train_width as u32,
                    train_height as u32,
                );
                if let Some(_train_going_left) = self.v_edges[r][c].train_to_a {
                    canvas.copy_ex(&gs.train, None, rect, 270.0, None, false, false)?;
                }
                if let Some(_train_going_right) = self.v_edges[r][c].train_to_b {
                    canvas.copy_ex(&gs.train, None, rect, 90.0, None, false, false)?;
                }
            }
        }

        Ok(())
    }
}
