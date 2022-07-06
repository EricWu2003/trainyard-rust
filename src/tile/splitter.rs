use crate::color::Color;

use crate::tile::BorderState;

#[derive(Debug, Clone)]
pub struct Splitter {
    pub incoming_dir: usize,
    pub incoming_train: Option<Color>,
    pub train_going_left: Option<Color>,
    pub train_going_right: Option<Color>,
}

impl Splitter {
    pub fn new(dir: usize) -> Splitter {
        Splitter {
            incoming_dir: dir,
            incoming_train: None,
            train_going_left: None,
            train_going_right: None,
        }
    }

    pub fn accept_trains(&mut self, trains: BorderState) -> bool {
        for i in 0..4 {
            if i == self.incoming_dir {
                self.incoming_train = trains[i];
            } else {
                if trains[i] != None {
                    return false;
                }
            }
        }
        true
    }

    pub fn dispatch_trains(&mut self) -> BorderState {
        let mut border_state = [None, None, None, None];
        border_state[(self.incoming_dir + 1) % 4] = self.train_going_left;
        border_state[(self.incoming_dir + 3) % 4] = self.train_going_right;
        self.train_going_left = None;
        self.train_going_right = None;
        border_state
    }
    pub fn process_tick(&mut self) {
        if let Some(color) = self.incoming_train {
            self.incoming_train = None;
            match color {
                Color::Brown | Color::Blue | Color::Red | Color::Yellow => {
                    self.train_going_left = Some(color);
                    self.train_going_right = Some(color);
                }
                Color::Orange => {
                    self.train_going_left = Some(Color::Yellow);
                    self.train_going_right = Some(Color::Red);
                }
                Color::Purple => {
                    self.train_going_left = Some(Color::Blue);
                    self.train_going_right = Some(Color::Red);
                }
                Color::Green => {
                    self.train_going_left = Some(Color::Blue);
                    self.train_going_right = Some(Color::Yellow);
                }
            }
        }
    }
}
