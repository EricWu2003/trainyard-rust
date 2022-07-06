use crate::color::Color;

use crate::connection::Connection;
use crate::tile::BorderState;

#[derive(Debug, Clone)]
pub struct Painter {
    pub connection: Connection,
    pub color: Color,
    pub train_to_dir1: Option<Color>,
    pub train_to_dir2: Option<Color>,
}

impl Painter {
    pub fn new(conn: Connection, color: Color) -> Painter {
        Painter {
            connection: conn,
            color,
            train_to_dir1: None,
            train_to_dir2: None,
        }
    }

    pub fn accept_trains(&mut self, trains: BorderState) -> bool {
        for i in 0..4 {
            if i == self.connection.dir1 {
                self.train_to_dir2 = trains[i as usize];
            } else if i == self.connection.dir2 {
                self.train_to_dir1 = trains[i as usize];
            } else {
                if trains[i as usize] != None {
                    return false;
                }
            }
        }
        true
    }

    pub fn dispatch_trains(&mut self) -> BorderState {
        let mut border_state = [None, None, None, None];
        if let Some(c) = self.train_to_dir1 {
            border_state[self.connection.dir1 as usize] = Some(c);
        }
        if let Some(c) = self.train_to_dir2 {
            border_state[self.connection.dir2 as usize] = Some(c);
        }
        border_state
    }
    pub fn process_tick(&mut self) {
        if self.train_to_dir1 != None {
            self.train_to_dir1 = Some(self.color);
        }
        if self.train_to_dir2 != None {
            self.train_to_dir2 = Some(self.color);
        }
    }
}
