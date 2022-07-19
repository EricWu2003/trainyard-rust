use crate::color::Color;

use crate::tile::BorderState;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::sprites::GameSprites;
use crate::particle::ParticleList;
use crate::particle::shrinking_circle::ShrinkingCircle;

#[derive(Debug, Clone)]
pub struct Trainsink {
    pub desires: Vec<Option<Color>>,
    pub private_desires: Vec<Option<Color>>,
    pub incoming_trains: BorderState,
    pub border_state: [bool; 4],
    pub icon_rects: Vec<Rect>,
    pub rect: Option<Rect>,
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
            rect: None,
            icon_rects: vec![],
        }
    }

    pub fn accept_trains(&mut self, trains: BorderState) -> BorderState {
        // when accepting the trains, we update the private desires while keeping public desires static.
        // this happens when trains are at the edge of the trainsink (still haven't reached the center yet)

        let mut border_state = [None, None, None, None];

        for (dir, train) in trains.iter().enumerate() {
            if let Some(color) = *train {
                if !self.border_state[dir] {
                    border_state[dir] = Some(color);
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
                        border_state[dir] = Some(color);
                    }
                }
            }
        }
        border_state
    }

    pub fn process_tick(&mut self, gs: &GameSprites, p: &mut ParticleList) {
        // when processing the tick, we update our public desires
        // this happens when the trains reach the center of the trainsink
        for index in 0..self.desires.len() {
            if self.desires[index].is_some() && self.private_desires[index].is_none() {
                let color = self.desires[index].unwrap();
                p.push(Box::new(ShrinkingCircle::new(
                    self.icon_rects[index], 
                    color,
                )));
            }
        }
        
        self.desires = self.private_desires.clone();
        for train in self.incoming_trains {
            if let Some(color) = train {
                gs.play_train_sound(color);
            }
        }
        self.incoming_trains = [None,None,None,None]
    }

    pub fn dispatch_trains(&mut self) -> BorderState {
        [None, None, None, None]
    }

    pub fn is_satisfied(&self) -> bool {
        for possible_desire in &self.desires {
            if possible_desire.is_some() {
                return false;
            }
        }
        return true;
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);

        let plus_sign_width = (rect.width() as f64 * (52.0 / 96.0)) as i32;
        let plus_sign_height = (rect.height() as f64 * (52.0 / 96.0)) as i32;
        let num_cols;
        if self.desires.len() <= 1 {
            num_cols = 1;
        } else if self.desires.len() <= 4 {
            num_cols = 2;
        } else if self.desires.len() <= 9 {
            num_cols = 3;
        } else {
            num_cols = 4;
        }
        for i in 0..self.desires.len() {
            let curr_col = i % num_cols;
            let curr_row = i / num_cols;
            let scaled_plus_sign_width = plus_sign_width / num_cols as i32;
            let scaled_plus_sign_height = plus_sign_height / num_cols as i32;
            let x_pos = rect.x()
                + (rect.width() as i32 - plus_sign_width) / 2
                + curr_col as i32 * scaled_plus_sign_width;
            let y_pos = rect.y()
                + (rect.width() as i32 - plus_sign_height) / 2
                + curr_row as i32 * scaled_plus_sign_height;
            self.icon_rects.push(
                Rect::new(
                    x_pos,
                    y_pos,
                    scaled_plus_sign_width as u32,
                    scaled_plus_sign_height as u32,
                )
            );
            
        }
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
                canvas.copy_ex(&gs.atlas_color, gs.train, train_rect, rot, None, false, false)?;
            }

        }
        Ok(())
    }
}
