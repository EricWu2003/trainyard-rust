use macroquad::prelude::*;
use std::i32;

use crate::GameState;
use crate::connection::Connection;
use crate::levels::{Level, LevelInfo};
use crate::particle::ParticleList;
use crate::yard::{YardState, NextAction};
use crate::yard::{NUM_COLS, NUM_ROWS};
use crate::{levels::LevelManager, sprites::GameSprites, yard::Yard};
use crate::sprites::SoundType::ButtonPress;
use crate::utils::{point_in_rect, draw_texture_to_rect, find_min_f32};

const MAX_SPEED:f32 = 0.17;
const DOUBLE_CLICK_THRESHOLD:u32 = 34;
pub struct Gameplay {
    yard_rect: Rect,
    start_trains_rect: Rect,
    erase_rect: Rect,
    back_rect: Rect,
    status_rect: Rect,
    speed_slider_space_rect: Rect,
    speed_slider_rect: Rect,
    yard: Yard,
    particles: ParticleList,
    prev_mouse_r: i32,
    prev_mouse_c: i32,
    prev_min_dir: i32,
    speed: f32,
    is_erasing: bool,
    frame_count: u32,
    last_click_time: u32,
    speed_btn_drag_offset: Option<f32>,
}

impl Gameplay {
    pub fn new(rect: Rect, level_manager: &LevelManager, gs: &GameSprites) -> Gameplay {
        let yard_rect = Rect::new(rect.x, rect.y, rect.w, rect.w);

        let (x, y) = (yard_rect.x, yard_rect.y + yard_rect.h);
        let scale = yard_rect.w / 672.;
        // let ui_rect = Rect::new(x, y, 672. * scale, 202. * scale);
        let start_trains_rect =  Rect::new(x + 238. * scale, y + 10. * scale, 424. * scale, 104. * scale);
        let erase_rect = Rect::new(x+10. * scale,y+10.*scale,208.*scale,88.*scale);
        let back_rect = Rect::new(x+10. * scale,y+110.*scale,208.*scale,88.*scale);
        let status_rect = Rect::new(x+10.*scale,y+10.*scale,208.*scale,168.*scale);
        let speed_slider_space_rect = Rect::new(x+238.*scale,y+134.*scale,424.*scale,68.*scale);
        let initial_speed = 0.5*MAX_SPEED;

        // the speed button can move 424 - 136 = 288 pixels
        let speed_btn_offset = (initial_speed/MAX_SPEED) * 288.0 * scale;
        let speed_slider_rect = Rect::new(x+238.*scale + speed_btn_offset,y+134.*scale,136. *scale ,68.*scale);

        Gameplay {
            yard_rect,
            start_trains_rect,
            erase_rect,
            back_rect,
            status_rect,
            speed_slider_space_rect,
            speed_slider_rect,
            yard: Yard::new(&level_manager.get_level("Red Line"), yard_rect, gs),
            prev_mouse_c: -1,
            prev_mouse_r: -1,
            prev_min_dir: -1,
            speed: initial_speed,
            is_erasing: false,
            frame_count: 0,
            last_click_time: 0,
            speed_btn_drag_offset: None,
            particles: vec![],
        }
    }

    pub fn render(&self, gs: &GameSprites) {
        self.yard.render(gs);

        match self.yard.state {
            YardState::Drawing => {
                if !self.is_erasing {
                    draw_texture_to_rect(gs.btn_erase, self.erase_rect);
                } else {
                    draw_texture_to_rect(gs.btn_stop_erase, self.erase_rect);
                }
                draw_texture_to_rect(gs.btn_start_trains, self.start_trains_rect);
                draw_texture_to_rect(gs.btn_back, self.back_rect);
            },
            YardState::Crashed => {
                draw_texture_to_rect(gs.btn_status_crashed, self.status_rect);
                draw_texture_to_rect(gs.btn_back_to_drawing, self.start_trains_rect);
            },
            YardState::Playing {..} | YardState::Won => {
                draw_texture_to_rect(gs.btn_status_good, self.status_rect);
                draw_texture_to_rect(gs.btn_back_to_drawing, self.start_trains_rect);
            }
        }
        for particle in &self.particles {
            particle.render(gs);
        }

        draw_texture_to_rect(gs.space_for_speed_slider, self.speed_slider_space_rect);
        draw_texture_to_rect(gs.btn_speed, self.speed_slider_rect);
    }

    pub fn update(&mut self, gs: &mut GameSprites, game_state: &mut GameState) -> bool {
        // returns true if we need to end the program (break out of the main loop)
        let grid_width = self.yard_rect.w / NUM_COLS as f32;
        let grid_height = self.yard_rect.h / NUM_ROWS as f32;

                // Event::Quit { .. } => return true,
                 
                // Event::KeyDown {
                //     keycode: Some(Keycode::Escape),
                //     ..
                // } => {
                //     return true;
                // }

        let (x, y) = mouse_position();

        if is_mouse_button_pressed(MouseButton::Left) {            
            let mut finished_double_click = false;
            if point_in_rect(x, y, self.start_trains_rect){
                match self.yard.state {
                    YardState::Crashed => {
                        self.yard.reset_self(gs);
                        self.yard.state = YardState::Drawing;
                    },
                    YardState::Drawing => {
                        self.is_erasing = false;
                        self.yard.state = YardState::Playing {
                            num_ticks_elapsed: 1,
                            progress: 0.0,
                            next_step: NextAction::ProcessTick,
                        };
                    },
                    YardState::Playing {..} => {
                        self.yard.reset_self(gs);
                        self.yard.state = YardState::Drawing;
                    },
                    YardState::Won => {
                        *game_state = GameState::Menu;
                    },
                }
                gs.add_sound(ButtonPress);
            } else if point_in_rect(x, y, self.erase_rect) {
                match self.yard.state {
                    YardState::Drawing => {
                        self.is_erasing = !self.is_erasing;
                        gs.add_sound(ButtonPress);
                    },
                    _ => {},
                }
            } else if point_in_rect(x, y, self.back_rect){
                if self.yard.state == YardState::Drawing {
                    *game_state = GameState::Menu;
                    gs.add_sound(ButtonPress);
                }
            } else if point_in_rect(x, y, self.yard_rect) {
                if self.frame_count - self.last_click_time < DOUBLE_CLICK_THRESHOLD {
                    match self.yard.state {
                        YardState::Drawing => {
                            let (x, y) = (
                                x - self.yard_rect.x,
                                y - self.yard_rect.y,
                            );
                            let (c, r) = (x / grid_width, y / grid_height);
                            self.yard.switch_connections(r as usize, c as usize, gs);
                            finished_double_click = true;
                        },
                        _ => {},
                    }
                }
            }
            if !finished_double_click {
                self.last_click_time = self.frame_count;
            }

        }


        if is_mouse_button_down(MouseButton::Left)
            && point_in_rect(x, y, self.speed_slider_rect) 
        {
            if let Some(offset) = self.speed_btn_drag_offset {
                let mut new_x = x - offset;
                if new_x < self.speed_slider_space_rect.x {
                    new_x = self.speed_slider_space_rect.x;
                } else if new_x > self.speed_slider_space_rect.x + self.speed_slider_space_rect.w - self.speed_slider_rect.w {
                    new_x = self.speed_slider_space_rect.x + self.speed_slider_space_rect.w - self.speed_slider_rect.w
                }
                self.speed_slider_rect.x = new_x;
                self.speed = (new_x - self.speed_slider_space_rect.x) / (self.speed_slider_space_rect.w - self.speed_slider_rect.w)  * MAX_SPEED;
            }
            else  {
                self.speed_btn_drag_offset = Some(x - self.speed_slider_rect.x);
            }
            
        } else {
            self.speed_btn_drag_offset = None;
        }
    

        if self.yard.state == YardState::Drawing && !self.is_erasing {
            if is_mouse_button_down(MouseButton::Left) && point_in_rect(x, y, self.yard_rect) 
            {
                // handle adding a connection to the yard when the user is drawing:
                let (x, y) = (
                    x - self.yard_rect.x,
                    y - self.yard_rect.y,
                );
                let (c, r) = ((x / grid_width )as i32, (y / grid_height) as i32);

                let dist_to_left = x % grid_width;
                let dist_to_up = y % grid_height;
                let distances = [
                    dist_to_up,
                    grid_width - dist_to_left,
                    grid_height - dist_to_up,
                    dist_to_left,
                ];

                let min_dist: f32 = find_min_f32(&distances);
                let mut min_dir = distances.iter().position(|&x| x == min_dist).unwrap() as i32;
                if min_dist > grid_width / 4. {
                    min_dir = -1;
                }

                if self.prev_mouse_c == c && self.prev_mouse_r == r {
                    if self.prev_min_dir != min_dir && min_dir != -1 && self.prev_min_dir != -1 {
                        self.yard.add_connection(
                            r as usize,
                            c as usize,
                            Connection {
                                dir1: self.prev_min_dir as u8,
                                dir2: min_dir as u8,
                            },
                            gs,
                            &mut self.particles,
                        );
                    }
                }

                self.prev_mouse_c = c;
                self.prev_mouse_r = r;
                if min_dir != -1 {
                    self.prev_min_dir = min_dir;
                }
            } else {
                self.prev_mouse_c = -1;
                self.prev_mouse_r = -1;
                self.prev_min_dir = -1;
            }
        } else if self.yard.state == YardState::Drawing && self.is_erasing {
            if is_mouse_button_down(MouseButton::Left) && point_in_rect(x, y, self.yard_rect) {
                let (x, y) = (
                    x - self.yard_rect.x,
                    y - self.yard_rect.y,
                );
                let (c, r) = ((x / grid_width )as i32, (y / grid_height) as i32);

                self.yard.clear_connections(r as usize, c as usize, gs);
            }
        }

        // Update
        self.yard.update(self.speed, gs, &mut self.particles);
        self.frame_count += 1;

        for particle in &mut self.particles {
            particle.pass_one_frame();
        }
        self.particles.retain(|particle| particle.still_exists());

        false
    }

    pub fn get_state(&self) -> YardState {
        self.yard.state
    }

    pub fn set_rect(&mut self, rect: Rect, gs: &GameSprites) {
        let yard_rect = Rect::new(rect.x, rect.y, rect.w, rect.w);
        self.yard_rect = yard_rect;

        let (x, y) = (yard_rect.x, yard_rect.y + yard_rect.h);
        let scale = yard_rect.w / 672.;

        self.start_trains_rect =  Rect::new(x + 238. * scale, y + 10. * scale, 424. * scale, 104. * scale);
        self.erase_rect = Rect::new(x+10. * scale,y+10.*scale,208.*scale,88.*scale);
        self.back_rect = Rect::new(x+10. * scale,y+110.*scale,208.*scale,88.*scale);
        self.status_rect = Rect::new(x+10.*scale,y+10.*scale,208.*scale,168.*scale);
        self.speed_slider_space_rect = Rect::new(x+238.*scale,y+134.*scale,424.*scale,68.*scale);


        let speed_btn_offset = (self.speed/MAX_SPEED) * 288.0 * scale;
        self.speed_slider_rect = Rect::new(x+238.*scale + speed_btn_offset,y+134.*scale,136. *scale ,68.*scale);

        self.yard.set_rect(yard_rect, gs);
    }

    pub fn reset_yard_from_level(&mut self, level: &Level, gs: &GameSprites) {
        self.yard = Yard::new(level, self.yard_rect, gs);
    }
    pub fn get_current_progress(&self) -> LevelInfo {
        self.yard.get_current_progress()
    }
}
