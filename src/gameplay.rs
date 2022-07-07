use sdl2::{rect::Rect, render::WindowCanvas, EventPump};
use crate::{yard::Yard, sprites::GameSprites, levels::LevelManager};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::connection::Connection;
use crate::yard::{NUM_COLS, NUM_ROWS};
use crate::yard::YardState;


pub struct Gameplay{
    rect: Rect,
    yard_rect: Rect,
    yard: Yard,
    prev_mouse_r: i32,
    prev_mouse_c: i32,
    prev_min_dir: i32,
    speed: f64,
}


impl Gameplay {
    pub fn new(rect: Rect, level_manager: &LevelManager) -> Gameplay{
        Gameplay{
            rect,
            yard_rect: rect,
            yard: Yard::from(level_manager.get_level("Halifax", "Handlebars")),
            prev_mouse_c: -1,
            prev_mouse_r: -1,
            prev_min_dir: -1,
            speed: 0.10,
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, gs: &mut GameSprites) -> Result<(), String> {
        self.yard.render(canvas, &self.rect, gs)?;
        Ok(())
    }

    pub fn update(&mut self, event_pump: &mut EventPump) -> bool {
        // returns true if we need to end the program (break out of the main loop)
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    return true
                }
                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
                } => {
                    self.yard.state = YardState::Playing {
                        num_ticks_elapsed: 0,
                        progress: 0.0,
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return true;
                }
                _ => {}
            }
        }
        if self.yard.state == YardState::Drawing {
            let mouse_state = event_pump.mouse_state();
            if mouse_state.left() {
                let grid_width = self.yard_rect.width() as i32 / NUM_COLS as i32;
                let grid_height = self.yard_rect.height() as i32 / NUM_ROWS as i32;

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
                if min_dist > grid_width / 7 {
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
                        )
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
        }
        // Update
        self.yard.update(self.speed);
        false
    }

    pub fn get_state(&self) -> YardState {
        self.yard.state
    }
}