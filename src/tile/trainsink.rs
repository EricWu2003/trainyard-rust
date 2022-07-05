use crate::color::Color;

use crate::tile::BorderState;

#[derive(Debug)]
pub struct Trainsink {
    pub desires: Vec<Option<Color>>,
    pub border_state: [bool; 4],
}

impl Trainsink {
    pub fn new(desires: Vec<Color>, border_state: [bool; 4]) -> Trainsink {
        Trainsink {
            desires: desires.into_iter().map(Some).collect(),
            border_state,
        }
    }

    pub fn accept_trains(&mut self, trains: BorderState) -> bool {
        for i in 0..4 {
            if let Some(color) = trains[i] {
                if !self.border_state[i] {
                    return false;
                } else {
                    let mut found = false;
                    for i in 0..self.desires.len() {
                        if self.desires[i] == Some(color) {
                            found = true;
                            self.desires[i] = None;
                            break;
                        }
                    }
                    if !found {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn dispatch_trains(&mut self) -> BorderState {
        [None, None, None, None]
    }
}
