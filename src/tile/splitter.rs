use macroquad::audio::play_sound_once;
use macroquad::prelude::*;
use crate::color::Color;
use crate::tile::BorderState;
use crate::sprites::GameSprites;
use crate::particle::ParticleList;
use crate::particle::splitter_particle::SplitterParticle;

use std::f32::consts::PI;
#[derive(Debug, Clone)]
pub struct Splitter {
    pub incoming_dir: u8,
    pub incoming_train: Option<Color>,
    pub train_going_left: Option<Color>,
    pub train_going_right: Option<Color>,
    pub rect: Option<Rect>,
}

impl Splitter {
    pub fn new(dir: u8) -> Splitter {
        Splitter {
            incoming_dir: dir,
            incoming_train: None,
            train_going_left: None,
            train_going_right: None,
            rect: None,
        }
    }

    pub fn accept_trains(&mut self, trains: BorderState) -> BorderState {
        let mut border_state = [None, None, None, None];
        for (dir, train) in trains.iter().enumerate() {
            if dir as u8 == self.incoming_dir {
                self.incoming_train = *train;
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
        let left_exit_dir = ((self.incoming_dir + 1) % 4) as usize;
        let right_exit_dir = ((self.incoming_dir + 3) % 4) as usize;
        border_state[left_exit_dir] = self.train_going_left;
        border_state[right_exit_dir] = self.train_going_right;
        self.train_going_left = None;
        self.train_going_right = None;
        border_state
    }

    pub fn process_tick(&mut self, gs: &GameSprites, p: &mut ParticleList) {
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
            p.push(Box::new(SplitterParticle::new(
                self.rect.unwrap(),
                self.incoming_dir,
            )));
            play_sound_once(gs.sl_splitter);
        }
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }

    pub fn render_trains(&self, gs: &GameSprites, progress: f32) { 
        let rect = self.rect.unwrap();
        let train_width = rect.w * (32.0 / 96.0);
        let train_height = rect.h * (57.0 / 96.0);

        let outgoing_left_dir = (self.incoming_dir + 1) % 4;
        let outgoing_right_dir = (self.incoming_dir + 3) % 4;

        if let Some(color) = self.incoming_train {
            // gs.set_color(color);
            let train_center_x;
            let train_center_y;
            let rot;
            if self.incoming_dir == 0 {
                train_center_x = rect.x + (rect.w/2.);
                train_center_y = rect.y + (rect.h * progress/2.0);
                rot = PI;
            } else if self.incoming_dir == 1 {
                train_center_x = rect.x + (rect.w * (1.0 - progress/2.0));
                train_center_y = rect.y + (rect.h/2.);
                rot = 3.*PI/2.;
            } else if self.incoming_dir == 2 {
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
                WHITE,
                DrawTextureParams { dest_size: None, source: None, rotation: rot, flip_x: false, flip_y: false, pivot: None }
            );
        }
        if let Some(color) = self.train_going_left {
            // gs.set_color(color);
            let train_center_x;
            let train_center_y;
            let rot;
            if outgoing_left_dir == 2 {
                train_center_x = rect.x + (rect.w/2.);
                train_center_y = rect.y + (rect.h * progress/2.0);
                rot = PI;
            } else if outgoing_left_dir == 3 {
                train_center_x = rect.x + (rect.w * (1.0 - progress/2.0));
                train_center_y = rect.y + (rect.h/2.);
                rot = 3.*PI/2.;
            } else if outgoing_left_dir == 0 {
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
                WHITE,
                DrawTextureParams { dest_size: None, source: None, rotation: rot, flip_x: false, flip_y: false, pivot: None }
            );
        }

        if let Some(color) = self.train_going_right {
            // gs.set_color(color);
            let train_center_x;
            let train_center_y;
            let rot;
            if outgoing_right_dir == 2 {
                train_center_x = rect.x + (rect.w/2.);
                train_center_y = rect.y + (rect.h * progress/2.0);
                rot = PI;
            } else if outgoing_right_dir == 3 {
                train_center_x = rect.x + (rect.w * (1.0 - progress/2.0));
                train_center_y = rect.y + (rect.h/2.);
                rot = 3.*PI/2.;
            } else if outgoing_right_dir == 0 {
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
                WHITE,
                DrawTextureParams { dest_size: None, source: None, rotation: rot, flip_x: false, flip_y: false, pivot: None }
            );
        }
    }
}
