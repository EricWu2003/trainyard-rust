use crate::color::Color;

use crate::tile::BorderState;

#[derive(Debug, Clone)]
pub struct Trainsource {
    pub trains: Vec<Option<Color>>,
    pub dir: usize,
}

impl Trainsource {
    pub fn new(trains: Vec<Color>, dir: usize) -> Trainsource {
        Trainsource {
            trains: trains.into_iter().map(Some).collect(),
            dir,
        }
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
        for i in 0..self.trains.len() {
            if let Some(color) = self.trains[i] {
                border_state[self.dir] = Some(color);
                self.trains[i] = None;
                break;
            }
        }
        border_state
    }

    pub fn is_empty(&self) -> bool {
        for i in 0..self.trains.len() {
            if self.trains[i] != None {
                return false;
            }
        }
        return true;
    }
}
