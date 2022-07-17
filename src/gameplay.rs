use std::i32;

use crate::connection::Connection;
use crate::particle::ParticleList;
use crate::yard::{YardState, NextAction};
use crate::yard::{NUM_COLS, NUM_ROWS};
use crate::{levels::LevelManager, sprites::GameSprites, yard::Yard};
use crate::utils::{point_in_rect, mouse_state_in_rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{rect::Rect, render::WindowCanvas, EventPump};

const MAX_SPEED:f64 = 0.17;
const DOUBLE_CLICK_THRESHOLD:u32 = 34;
pub struct Gameplay {
    yard_rect: Rect,
    ui_rect: Rect,
    start_trains_rect: Rect,
    erase_rect: Rect,
    speed_slider_space_rect: Rect,
    speed_slider_rect: Rect,
    yard: Yard,
    particles: ParticleList,
    prev_mouse_r: i32,
    prev_mouse_c: i32,
    prev_min_dir: i32,
    speed: f64,
    is_erasing: bool,
    frame_count: u32,
    last_click_time: u32,
    speed_btn_drag_offset: Option<i32>,
}

impl Gameplay {
    pub fn new(rect: Rect, level_manager: &LevelManager) -> Gameplay {
        let (x, y) = (rect.x(), rect.y() + rect.height() as i32);
        let ui_rect = Rect::new(x, y, 672, 202);
        let start_trains_rect =  Rect::new(x + 238, y + 10, 424, 104);
        let erase_rect = Rect::new(x+10,y+10,208,88);
        let speed_slider_space_rect = Rect::new(x+238,y+134,424,68);
        let initial_speed = 0.5*MAX_SPEED;

        // the speed button can move 424 - 136 = 288 pixels
        let speed_btn_offset = ((initial_speed/MAX_SPEED) * 288.0) as i32;
        let speed_slider_rect = Rect::new(x+238 + speed_btn_offset,y+134,136 ,68);

        Gameplay {
            yard_rect: rect,
            ui_rect,
            start_trains_rect,
            erase_rect,
            speed_slider_space_rect,
            speed_slider_rect,
            yard: Yard::new(level_manager.get_level( "Round The Twist"), rect),
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

    pub fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        self.yard.render(canvas, &self.yard_rect, gs)?;
        let (x,y) = (self.ui_rect.x(), self.ui_rect.y());

        match self.yard.state {
            YardState::Drawing => {
                if !self.is_erasing {
                    canvas.copy(&gs.atlas, gs.btn_erase, self.erase_rect)?;
                } else {
                    canvas.copy(&gs.atlas, gs.btn_stop_erase, self.erase_rect)?;
                }
                canvas.copy(&gs.atlas, gs.btn_start_trains, self.start_trains_rect)?;
            },
            YardState::Crashed => {
                canvas.copy(&gs.atlas, gs.btn_status_crashed, Rect::new(x+10,y+10,208,168))?;
                canvas.copy(&gs.atlas, gs.btn_back_to_drawing, self.start_trains_rect)?;
            },
            YardState::Playing {..} | YardState::Won => {
                canvas.copy(&gs.atlas, gs.btn_status_good, Rect::new(x+10,y+10,208,168))?;
                canvas.copy(&gs.atlas, gs.btn_back_to_drawing, self.start_trains_rect)?;
            }
        }
        for particle in &self.particles {
            particle.render(canvas, gs)?;
        }
        
        
        canvas.copy(&gs.atlas, gs.space_for_speed_slider, self.speed_slider_space_rect)?;
        canvas.copy(&gs.atlas, gs.btn_speed, self.speed_slider_rect)?;
        Ok(())
    }

    pub fn update(&mut self, event_pump: &mut EventPump, gs: &GameSprites) -> bool {
        // returns true if we need to end the program (break out of the main loop)
        let mouse_state = event_pump.mouse_state();
        let grid_width = self.yard_rect.width() as i32 / NUM_COLS as i32;
        let grid_height = self.yard_rect.height() as i32 / NUM_ROWS as i32;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return true,
                 
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return true;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    let mut finished_double_click = false;
                    if point_in_rect(x, y, self.start_trains_rect){
                        match self.yard.state {
                            YardState::Crashed => {
                                self.yard.reset_self();
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
                                self.yard.reset_self();
                                self.yard.state = YardState::Drawing;
                            },
                            YardState::Won => {},
                        }
                        gs.sl.play(&gs.sl_button_press);
                    } else if point_in_rect(x, y, self.erase_rect) {
                        match self.yard.state {
                            YardState::Drawing => {
                                self.is_erasing = !self.is_erasing;
                                gs.sl.play(&gs.sl_button_press);
                            },
                            _ => {},
                        }
                    } else if point_in_rect(x, y, self.yard_rect) {
                        if self.frame_count - self.last_click_time < DOUBLE_CLICK_THRESHOLD {
                            match self.yard.state {
                                YardState::Drawing => {
                                    let (x, y) = (
                                        mouse_state.x() - self.yard_rect.x(),
                                        mouse_state.y() - self.yard_rect.y(),
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
                _ => {}
            }
        }

        if mouse_state.left() 
            && mouse_state_in_rect(mouse_state, self.speed_slider_rect) 
        {
            if let Some(offset) = self.speed_btn_drag_offset {
                let mut new_x = mouse_state.x() - offset;
                if new_x < self.speed_slider_space_rect.x() {
                    new_x = self.speed_slider_space_rect.x();
                } else if new_x > self.speed_slider_space_rect.x() + self.speed_slider_space_rect.width() as i32 - self.speed_slider_rect.width() as i32 {
                    new_x = self.speed_slider_space_rect.x() + self.speed_slider_space_rect.width() as i32 - self.speed_slider_rect.width() as i32;
                }
                self.speed_slider_rect = Rect::new(new_x, self.speed_slider_rect.y(), self.speed_slider_rect.width(), self.speed_slider_rect.height());
                self.speed = (new_x - self.speed_slider_space_rect.x()) as f64 / (self.speed_slider_space_rect.width() - self.speed_slider_rect.width()) as f64 * MAX_SPEED;
            }
            else  {
                self.speed_btn_drag_offset = Some(mouse_state.x() - self.speed_slider_rect.x());
            }
            
        } else {
            self.speed_btn_drag_offset = None;
        }
    

        if self.yard.state == YardState::Drawing && !self.is_erasing {
            if mouse_state.left() && mouse_state_in_rect(mouse_state, self.yard_rect)
            {
                // handle adding a connection to the yard when the user is drawing:
                let (x, y) = (
                    mouse_state.x() - self.yard_rect.x(),
                    mouse_state.y() - self.yard_rect.y(),
                );
                let (c, r) = (x / grid_width, y / grid_height);

                let dist_to_left = x % grid_width;
                let dist_to_up = y % grid_height;
                let distances = [
                    dist_to_up,
                    grid_width - dist_to_left,
                    grid_height - dist_to_up,
                    dist_to_left,
                ];
                let min_dist = *distances.iter().min().unwrap();
                let mut min_dir = distances.iter().position(|&x| x == min_dist).unwrap() as i32;
                if min_dist > grid_width / 4 {
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
            if mouse_state.left()
                && mouse_state_in_rect(mouse_state, self.yard_rect)
            {
                let (x, y) = (
                    mouse_state.x() - self.yard_rect.x(),
                    mouse_state.y() - self.yard_rect.y(),
                );
                let (c, r) = (x / grid_width, y / grid_height);

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
}
