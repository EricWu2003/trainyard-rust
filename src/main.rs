pub mod color;
pub mod connection;
pub mod edge;
pub mod sprites;
pub mod tile;
pub mod yard;
use crate::sprites::GameSprites;
use crate::yard::Yard;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use sdl2::rect::Rect;
use std::time::Duration;

fn main() -> Result<(), String> {
    let mut yard = Yard::new();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    // Leading "_" tells Rust that this is an unused variable that we don't care about. It has to
    // stay unused because if we don't have any variable at all then Rust will treat it as a
    // temporary value and drop it right away!
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("game tutorial", 672, 672)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let game_sprites = GameSprites::new(&texture_creator)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut i = 0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
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

        // Update
        i = (i + 1) % 255;

        // Render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        yard.render(&mut canvas, Rect::new(0, 0, 336, 336), &game_sprites)?;
        // canvas.copy(&game_sprites.tracktile_blank, None, Rect::new(0,0,96,96))?;

        canvas.present();

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
