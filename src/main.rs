pub mod color;
pub mod connection;
pub mod edge;
pub mod levels;
pub mod sprites;
pub mod tile;
pub mod yard;
use crate::levels::LEVEL_MANAGER;
use crate::sprites::GameSprites;
use crate::yard::Yard;

use connection::Connection;
use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use sdl2::rect::Rect;
use std::time::Duration;

use yard::{NUM_COLS, NUM_ROWS};

fn main() -> Result<(), String> {
    let mut yard: Yard;
    levels::initialize();
    unsafe {
        yard = Yard::from(LEVEL_MANAGER.get_level("Calgary", "Rainbow"));
    }

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("game tutorial", 700, 800)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let mut game_sprites = GameSprites::new(&texture_creator)?;

    let mut event_pump = sdl_context.event_pump()?;
    let yard_rect = Rect::new(14, 40, 672, 672);

    let mut prev_mouse_r = -1;
    let mut prev_mouse_c = -1;
    let mut prev_min_dir = -1;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
                } => {
                    yard.process_tick();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
        let mouse_state = event_pump.mouse_state();
        if mouse_state.left() {
            let grid_width = yard_rect.width() as i32 / NUM_COLS as i32;
            let grid_height = yard_rect.height() as i32 / NUM_ROWS as i32;

            let (x, y) = (mouse_state.x() - yard_rect.x(), mouse_state.y() - yard_rect.y());
            let (c, r) = (
                x / grid_width,
                y / grid_height,
            );

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

            if prev_mouse_c == c && prev_mouse_r == r {
                if prev_min_dir != min_dir && min_dir != -1 && prev_min_dir != -1 {
                    yard.add_connection(
                        r as usize,
                        c as usize,
                        Connection {
                            dir1: prev_min_dir as u8,
                            dir2: min_dir as u8,
                        },
                    )
                }
            }

            prev_mouse_c = c;
            prev_mouse_r = r;
            if min_dir != -1 {
                prev_min_dir = min_dir;
            }
        } else {
            prev_mouse_c = -1;
            prev_mouse_r = -1;
            prev_min_dir = -1;
        }

        // Update

        // Render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        yard.render(&mut canvas, &yard_rect, &mut game_sprites)?;
        // canvas.copy(&game_sprites.tracktile_blank, None, Rect::new(0,0,96,96))?;

        canvas.present();

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }

    Ok(())
}
