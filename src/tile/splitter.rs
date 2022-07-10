use crate::color::Color;
use crate::tile::BorderState;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::sprites::GameSprites;

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

    pub fn render_trains(&self, canvas: &mut WindowCanvas, rect: &Rect, gs: &mut GameSprites, progress: f64) -> Result<(), String> { 
        let train_width = (rect.width() as f64 * (32.0 / 96.0)) as u32;
        let train_height = (rect.height() as f64 * (57.0 / 96.0)) as u32;

        let outgoing_left_dir = (self.incoming_dir + 1) % 4;
        let outgoing_right_dir = (self.incoming_dir + 3) % 4;

        if let Some(color) = self.incoming_train {
            gs.set_color(color);
            let train_center_x;
            let train_center_y;
            let rot;
            if self.incoming_dir == 0 {
                train_center_x = rect.x() + (rect.width()/2) as i32;
                train_center_y = rect.y() + (rect.height() as f64 * progress/2.0) as i32;
                rot = 180.0;
            } else if self.incoming_dir == 1 {
                train_center_x = rect.x() + (rect.width() as f64 * (1.0 - progress/2.0)) as i32;
                train_center_y = rect.y() + (rect.height()/2) as i32;
                rot = 270.0;
            } else if self.incoming_dir == 2 {
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
        if let Some(color) = self.train_going_left {
            gs.set_color(color);
            let train_center_x;
            let train_center_y;
            let rot;
            if outgoing_left_dir == 2 {
                train_center_x = rect.x() + (rect.width()/2) as i32;
                train_center_y = rect.y() + (rect.height() as f64 * progress/2.0) as i32;
                rot = 180.0;
            } else if outgoing_left_dir == 3 {
                train_center_x = rect.x() + (rect.width() as f64 * (1.0 - progress/2.0)) as i32;
                train_center_y = rect.y() + (rect.height()/2) as i32;
                rot = 270.0;
            } else if outgoing_left_dir == 0 {
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

        if let Some(color) = self.train_going_right {
            gs.set_color(color);
            let train_center_x;
            let train_center_y;
            let rot;
            if outgoing_right_dir == 2 {
                train_center_x = rect.x() + (rect.width()/2) as i32;
                train_center_y = rect.y() + (rect.height() as f64 * progress/2.0) as i32;
                rot = 180.0;
            } else if outgoing_right_dir == 3 {
                train_center_x = rect.x() + (rect.width() as f64 * (1.0 - progress/2.0)) as i32;
                train_center_y = rect.y() + (rect.height()/2) as i32;
                rot = 270.0;
            } else if outgoing_right_dir == 0 {
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
        
        Ok(())
    }

}
