use crate::color::Color;

use crate::tile::BorderState;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::sprites::GameSprites;

#[derive(Debug, Clone)]
pub struct Trainsink {
    pub desires: Vec<Option<Color>>,
    pub private_desires: Vec<Option<Color>>,
    pub incoming_trains: BorderState,
    pub border_state: [bool; 4],
}

impl Trainsink {
    pub fn new(desires: Vec<Color>, border_state: [bool; 4]) -> Trainsink {
        // The reason we keep two copies of desires is to account for the edge case where two trains of the
        // same color try to enter the trainsink at once, while the trainsink only wants one.
        let desires2 = desires.clone();
        Trainsink {
            desires: desires.into_iter().map(Some).collect(),
            private_desires: desires2.into_iter().map(Some).collect(),
            border_state,
            incoming_trains: [None,None,None,None],
        }
    }

    pub fn accept_trains(&mut self, trains: BorderState) -> bool {
        for dir in 0..4 {
            if let Some(color) = trains[dir] {
                if !self.border_state[dir] {
                    return false;
                } else {
                    let mut found = false;
                    for index in 0..self.private_desires.len() {
                        if self.private_desires[index] == Some(color) {
                            found = true;
                            self.incoming_trains[dir] = Some(color);
                            self.private_desires[index] = None;
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

    pub fn process_tick(&mut self) {
        self.desires = self.private_desires.clone();
        self.incoming_trains = [None,None,None,None]
    }

    pub fn dispatch_trains(&mut self) -> BorderState {
        [None, None, None, None]
    }

    pub fn is_satisfied(&self) -> bool {
        for i in 0..self.desires.len() {
            if self.desires[i] != None {
                return false;
            }
        }
        return true;
    }


    pub fn render_trains(&self, canvas: &mut WindowCanvas, rect: &Rect, gs: &mut GameSprites, progress: f64) -> Result<(), String> {
        let train_width = (rect.width() as f64 * (32.0 / 96.0)) as u32;
        let train_height = (rect.height() as f64 * (57.0 / 96.0)) as u32;

        for i in 0..4 {
            if let Some(color) = self.incoming_trains[i] {
                gs.set_color(color);
                let train_center_x;
                let train_center_y;
                let rot;
                if i == 0 {
                    train_center_x = rect.x() + (rect.width()/2) as i32;
                    train_center_y = rect.y() + (rect.height() as f64 * progress/2.0) as i32;
                    rot = 180.0;
                } else if i == 1 {
                    train_center_x = rect.x() + (rect.width() as f64 * (1.0 - progress/2.0)) as i32;
                    train_center_y = rect.y() + (rect.height()/2) as i32;
                    rot = 270.0;
                } else if i == 2 {
                    train_center_x = rect.x() + (rect.width()/2) as i32;
                    train_center_y = rect.y() + (rect.height() as f64 * (1.0 - progress/2.0)) as i32;
                    rot = 0.0;
                } else {
                    train_center_x = rect.x() + (rect.width() as f64 * progress/2.0) as i32;
                    train_center_y = rect.y() + (rect.height()/2) as i32;
                    rot = 90.0;
                }
                let train_rect = Rect::new(train_center_x - (train_width/2) as i32, train_center_y - (train_height/2) as i32, train_width, train_height);
                canvas.copy_ex(&gs.train, None, train_rect, rot, None, false, false)?;
            }

        }
        Ok(())
    }
}
