use macroquad::prelude::*;
use crate::color::Color;

use crate::connection::Connection;
use crate::tile::BorderState;

use crate::sprites::GameSprites;
use crate::sprites::SoundType;
use crate::particle::ParticleList;
use crate::particle::painter_particle::PainterParticle;

use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Painter {
    pub connection: Connection,
    pub color: Color,
    pub train_to_dir1: Option<Color>,
    pub train_to_dir2: Option<Color>,
    pub rect: Option<Rect>,
    scale: f32,
}

impl Painter {
    pub fn new(conn: Connection, color: Color) -> Painter {
        Painter {
            connection: conn,
            color,
            train_to_dir1: None,
            train_to_dir2: None,
            rect: None,
            scale: 1.,
        }
    }

    pub fn accept_trains(&mut self, trains: BorderState) -> BorderState {
        let mut border_state = [None, None, None, None];
        for (dir, train) in trains.iter().enumerate() {
            if dir as u8 == self.connection.dir1 {
                self.train_to_dir2 = *train;
            } else if dir as u8 == self.connection.dir2 {
                self.train_to_dir1 = *train;
            } else {
                if train.is_some() {
                    border_state[dir] = *train;
                }
            }
        }
        border_state
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
    pub fn process_tick(&mut self, gs: &mut GameSprites, p: &mut ParticleList) {
        if self.train_to_dir1.is_some() {
            self.train_to_dir1 = Some(self.color);
            gs.add_sound(SoundType::Painter);
        }
        if self.train_to_dir2.is_some() {
            self.train_to_dir2 = Some(self.color);
            gs.add_sound(SoundType::Painter);
        }
        if self.train_to_dir1.is_some() || self.train_to_dir2.is_some() {
            p.push(Box::new(PainterParticle::new(
                self.rect.unwrap(),
                self.color,
            )));
        }
    }

    pub fn set_rect(&mut self, rect: Rect, gs: &GameSprites) {
        self.rect = Some(rect);
        self.scale = rect.w / gs.tracktile_blank.width();
    }

    pub fn render_trains(&self, gs: &GameSprites, progress: f32) {
        let rect = self.rect.unwrap();
        let train_width = self.scale * gs.train.width();
        let train_height = self.scale * gs.train.height();

        let dest_size = Some(Vec2::new(train_width, train_height));
        
        if progress <= 1.0 {
            // render the incoming trains
            if let Some(color) = self.train_to_dir2 {
                let train_center_x;
                let train_center_y;
                let rot;
                if self.connection.dir1 == 0 {
                    train_center_x = rect.x + (rect.w/2.) ;
                    train_center_y = rect.y + (rect.h * progress/2.0) ;
                    rot = PI;
                } else if self.connection.dir1 == 1 {
                    train_center_x = rect.x + (rect.w * (1.0 - progress/2.0)) ;
                    train_center_y = rect.y + (rect.h/2.) ;
                    rot = 3.*PI/2.;
                } else if self.connection.dir1 == 2 {
                    train_center_x = rect.x + (rect.w/2.) ;
                    train_center_y = rect.y + (rect.h * (1.0 - progress/2.0)) ;
                    rot = 0.0;
                } else {
                    train_center_x = rect.x + (rect.w * progress/2.0) ;
                    train_center_y = rect.y + (rect.h/2.) ;
                    rot = PI/2.;
                }
                draw_texture_ex(
                    gs.train,
                    train_center_x - (train_width/2.),
                    train_center_y - (train_height/2.),
                    color.get_color(),
                    DrawTextureParams { dest_size, source: None, rotation: rot, flip_x: false, flip_y: false, pivot: None }
                );
            }
            if let Some(color) = self.train_to_dir1 {
                let train_center_x;
                let train_center_y;
                let rot;
                if self.connection.dir2 == 0 {
                    train_center_x = rect.x + (rect.w/2.);
                    train_center_y = rect.y + (rect.h * progress/2.0);
                    rot = PI;
                } else if self.connection.dir2 == 1 {
                    train_center_x = rect.x + (rect.w * (1.0 - progress/2.0));
                    train_center_y = rect.y + (rect.h/2.);
                    rot = 3.*PI/2.;
                } else if self.connection.dir2 == 2 {
                    train_center_x = rect.x + (rect.w/2.);
                    train_center_y = rect.y + (rect.h * (1.0 - progress/2.0));
                    rot = 0.0;
                } else {
                    train_center_x = rect.x + (rect.w * progress/2.0);
                    train_center_y = rect.y + (rect.h/2.);
                    rot = PI/2.;
                }
                draw_texture_ex(
                    gs.train,
                    train_center_x - (train_width/2.),
                    train_center_y - (train_height/2.),
                    color.get_color(),
                    DrawTextureParams { dest_size, source: None, rotation: rot, flip_x: false, flip_y: false, pivot: None }
                );
            }
        } else {
            //render the outgoing trains


            if let Some(color) = self.train_to_dir1 {
                let train_center_x;
                let train_center_y;
                let rot;
                if self.connection.dir1 == 2 {
                    train_center_x = rect.x + (rect.w/2.);
                    train_center_y = rect.y + (rect.h * progress/2.0);
                    rot = PI;
                } else if self.connection.dir1 == 3 {
                    train_center_x = rect.x + (rect.w * (1.0 - progress/2.0));
                    train_center_y = rect.y + (rect.h/2.);
                    rot = 3.*PI/2.;
                } else if self.connection.dir1 == 0 {
                    train_center_x = rect.x + (rect.w/2.);
                    train_center_y = rect.y + (rect.h * (1.0 - progress/2.0));
                    rot = 0.0;
                } else {
                    train_center_x = rect.x + (rect.w * progress/2.0);
                    train_center_y = rect.y + (rect.h/2.);
                    rot = PI/2.;
                }
                draw_texture_ex(
                    gs.train,
                    train_center_x - (train_width/2.),
                    train_center_y - (train_height/2.),
                    color.get_color(),
                    DrawTextureParams { dest_size, source: None, rotation: rot, flip_x: false, flip_y: false, pivot: None }
                );
            }
            if let Some(color) = self.train_to_dir2 {
                let train_center_x;
                let train_center_y;
                let rot;
                if self.connection.dir2 == 2 {
                    train_center_x = rect.x + (rect.w/2.);
                    train_center_y = rect.y + (rect.h * progress/2.0);
                    rot = PI;
                } else if self.connection.dir2 == 3 {
                    train_center_x = rect.x + (rect.w * (1.0 - progress/2.0));
                    train_center_y = rect.y + (rect.h/2.);
                    rot = 3.*PI/2.;
                } else if self.connection.dir2 == 0 {
                    train_center_x = rect.x + (rect.w/2.);
                    train_center_y = rect.y + (rect.h * (1.0 - progress/2.0));
                    rot = 0.0;
                } else {
                    train_center_x = rect.x + (rect.w * progress/2.0);
                    train_center_y = rect.y + (rect.h/2.);
                    rot = PI/2.;
                }
                draw_texture_ex(
                    gs.train,
                    train_center_x - (train_width/2.),
                    train_center_y - (train_height/2.),
                    color.get_color(),
                    DrawTextureParams { dest_size, source: None, rotation: rot, flip_x: false, flip_y: false, pivot: None }
                );
            }
        }
    }
}
