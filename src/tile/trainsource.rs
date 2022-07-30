use macroquad::prelude::*;
use crate::color::Color;

use crate::tile::BorderState;
use crate::sprites::GameSprites;
use crate::particle::ParticleList;
use crate::particle::shrinking_plus::ShrinkingPlus;

use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Trainsource {
    pub trains: Vec<Option<Color>>,
    pub dir: u8,
    pub outgoing_train: Option<Color>,
    pub icon_rects: Vec<Rect>,
    pub rect: Option<Rect>,
    pub scale: f32,
}

impl Trainsource {
    pub fn new(trains: Vec<Color>, dir: u8) -> Trainsource {
        Trainsource {
            trains: trains.into_iter().map(Some).collect(),
            dir,
            outgoing_train: None,
            rect: None,
            icon_rects: vec![],
            scale: 1.,
        }
    }

    pub fn accept_trains(&self, trains: BorderState) -> BorderState {
        return trains;
    }

    pub fn process_tick(&mut self, p: &mut ParticleList) {
        for (index, train) in self.trains.iter().enumerate() {
            if let Some(color) = *train {
                self.outgoing_train = Some(color);
                self.trains[index] = None;
                p.push(Box::new(ShrinkingPlus::new(
                    self.icon_rects[index], 
                    color,
                )));
                return;
            }
        }
    }

    pub fn dispatch_trains(&mut self) -> BorderState {
        let mut border_state = [None, None, None, None];
        border_state[self.dir as usize] = self.outgoing_train;
        self.outgoing_train = None;
        border_state
    }

    pub fn is_empty(&self) -> bool {
        for train in &self.trains {
            if train.is_some() {
                return false;
            }
        }
        return true;
    }

    pub fn set_rect(&mut self, rect: Rect, gs: &GameSprites) {
        self.rect = Some(rect);
        self.scale = rect.w / gs.tracktile_blank.width();

        let plus_sign_width = self.scale * gs.plus_sign.width();
        let plus_sign_height = self.scale * gs.plus_sign.height();
        let num_cols;
        if self.trains.len() <= 1 {
            num_cols = 1;
        } else if self.trains.len() <= 4 {
            num_cols = 2;
        } else if self.trains.len() <= 9 {
            num_cols = 3;
        } else {
            num_cols = 4;
        }
        for i in 0..self.trains.len() {
            let curr_col = i % num_cols;
            let curr_row = i / num_cols;
            let scaled_plus_sign_width = plus_sign_width / num_cols as f32;
            let scaled_plus_sign_height = plus_sign_height / num_cols as f32;
            let x_pos = rect.x
                + (rect.w - plus_sign_width) / 2.
                + curr_col as f32 * scaled_plus_sign_width;
            let y_pos = rect.y
                + (rect.w - plus_sign_height) / 2.
                + curr_row as f32 * scaled_plus_sign_height;
            self.icon_rects.push(
                Rect::new(
                    x_pos,
                    y_pos,
                    scaled_plus_sign_width,
                    scaled_plus_sign_height,
                )
            );
            
        }
    }


    pub fn render_trains(&self, gs: &GameSprites, progress: f32) {
        let rect = self.rect.unwrap();
        if let Some(color) = self.outgoing_train {
            let train_width = gs.train.width() * self.scale;
            let train_height = gs.train.height() * self.scale;
            let train_center_x;
            let train_center_y;
            let rot;

            if self.dir == 2 {
                train_center_x = rect.x + (rect.w/2.);
                train_center_y = rect.y + (rect.h * progress/2.0);
                rot = PI;
            } else if self.dir == 3 {
                train_center_x = rect.x + (rect.w * (1.0 - progress/2.0));
                train_center_y = rect.y + (rect.h/2.);
                rot = 3. * PI/2.;
            } else if self.dir == 0 {
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
                DrawTextureParams {
                    dest_size: Some(Vec2::new(train_width, train_height)),
                    source: None,
                    rotation: rot,
                    flip_x: false,
                    flip_y: false,
                    pivot: None
                }
            );

        }
    }
}
