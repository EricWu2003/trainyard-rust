use crate::color::Color;

use crate::tile::BorderState;


#[derive(Debug)]
pub struct Trainsource {
    pub trains: Vec<Color>,
    pub n_trains_init: u8,
    pub dir: usize,
}

impl Trainsource {
    pub fn new(trains: Vec<Color>, dir: usize) -> Trainsource {
        let n_trains_init = trains.len() as u8;
        Trainsource { trains, n_trains_init, dir }
    }

    pub fn accept_trains(&self, trains: BorderState) -> bool {
        for i in 0..4 {
            if trains[i] != None {
                return false;
            }
        }
        true
    }

    pub fn dispatch_trains(&mut self) -> BorderState {
        let mut border_state = [None, None, None, None];
        if let Some(color) = self.trains.pop() {
            border_state[self.dir] = Some(color);
        }
        border_state
    }
}
