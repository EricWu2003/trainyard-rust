use crate::color::Color;

use crate::connection::Connection;
use crate::tile::BorderState;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::sprites::GameSprites;
use crate::particle::ParticleList;
use crate::particle::painter_particle::PainterParticle;

#[derive(Debug, Clone)]
pub struct Painter {
    pub connection: Connection,
    pub color: Color,
    pub train_to_dir1: Option<Color>,
    pub train_to_dir2: Option<Color>,
    rect: Option<Rect>,
}

impl Painter {
    pub fn new(conn: Connection, color: Color) -> Painter {
        Painter {
            connection: conn,
            color,
            train_to_dir1: None,
            train_to_dir2: None,
            rect: None
        }
    }

    pub fn accept_trains(&mut self, trains: BorderState) -> bool {
        for (dir, train) in trains.iter().enumerate() {
            if dir as u8 == self.connection.dir1 {
                self.train_to_dir2 = *train;
            } else if dir as u8 == self.connection.dir2 {
                self.train_to_dir1 = *train;
            } else {
                if train.is_some() {
                    return false.clone();
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
    pub fn process_tick(&mut self, gs: &GameSprites, p: &mut ParticleList) {
        if self.train_to_dir1.is_some() {
            self.train_to_dir1 = Some(self.color);
            gs.sl.play(&gs.sl_painter);
        }
        if self.train_to_dir2.is_some() {
            self.train_to_dir2 = Some(self.color);
            gs.sl.play(&gs.sl_painter);
        }
        if self.train_to_dir1.is_some() || self.train_to_dir2.is_some() {
            p.push(Box::new(PainterParticle::new(
                self.rect.unwrap(),
                self.color,
            )));
        }
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }

    pub fn render_trains(&self, canvas: &mut WindowCanvas, rect: &Rect, gs: &mut GameSprites, progress: f64) -> Result<(), String> {
        let train_width = (rect.width() as f64 * (32.0 / 96.0)) as u32;
        let train_height = (rect.height() as f64 * (57.0 / 96.0)) as u32;
        
        if progress <= 1.0 {
            // render the incoming trains
            if let Some(color) = self.train_to_dir2 {
                gs.set_color(color);
                let train_center_x;
                let train_center_y;
                let rot;
                if self.connection.dir1 == 0 {
                    train_center_x = rect.x() + (rect.width()/2) as i32;
                    train_center_y = rect.y() + (rect.height() as f64 * progress/2.0) as i32;
                    rot = 180.0;
                } else if self.connection.dir1 == 1 {
                    train_center_x = rect.x() + (rect.width() as f64 * (1.0 - progress/2.0)) as i32;
                    train_center_y = rect.y() + (rect.height()/2) as i32;
                    rot = 270.0;
                } else if self.connection.dir1 == 2 {
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
            if let Some(color) = self.train_to_dir1 {
                gs.set_color(color);
                let train_center_x;
                let train_center_y;
                let rot;
                if self.connection.dir2 == 0 {
                    train_center_x = rect.x() + (rect.width()/2) as i32;
                    train_center_y = rect.y() + (rect.height() as f64 * progress/2.0) as i32;
                    rot = 180.0;
                } else if self.connection.dir2 == 1 {
                    train_center_x = rect.x() + (rect.width() as f64 * (1.0 - progress/2.0)) as i32;
                    train_center_y = rect.y() + (rect.height()/2) as i32;
                    rot = 270.0;
                } else if self.connection.dir2 == 2 {
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
        } else {
            //render the outgoing trains


            if let Some(color) = self.train_to_dir1 {
                gs.set_color(color);
                let train_center_x;
                let train_center_y;
                let rot;
                if self.connection.dir1 == 2 {
                    train_center_x = rect.x() + (rect.width()/2) as i32;
                    train_center_y = rect.y() + (rect.height() as f64 * progress/2.0) as i32;
                    rot = 180.0;
                } else if self.connection.dir1 == 3 {
                    train_center_x = rect.x() + (rect.width() as f64 * (1.0 - progress/2.0)) as i32;
                    train_center_y = rect.y() + (rect.height()/2) as i32;
                    rot = 270.0;
                } else if self.connection.dir1 == 0 {
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
            if let Some(color) = self.train_to_dir2 {
                gs.set_color(color);
                let train_center_x;
                let train_center_y;
                let rot;
                if self.connection.dir2 == 2 {
                    train_center_x = rect.x() + (rect.width()/2) as i32;
                    train_center_y = rect.y() + (rect.height() as f64 * progress/2.0) as i32;
                    rot = 180.0;
                } else if self.connection.dir2 == 3 {
                    train_center_x = rect.x() + (rect.width() as f64 * (1.0 - progress/2.0)) as i32;
                    train_center_y = rect.y() + (rect.height()/2) as i32;
                    rot = 270.0;
                } else if self.connection.dir2 == 0 {
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
