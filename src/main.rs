pub mod color;
pub mod connection;
pub mod edge;
pub mod levels;
pub mod sprites;
pub mod tile;
pub mod yard;
pub mod gameplay;
use crate::levels::LevelManager;
use crate::sprites::GameSprites;
use crate::yard::YardState;
use crate::gameplay::Gameplay
;
use sdl2::image::{self, InitFlag};
use sdl2::pixels::Color;

use sdl2::rect::Rect;
use std::time::Duration;


fn main() -> Result<(), String> {
    let level_manager = LevelManager::new();
    

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

    let mut gameplay = Gameplay::new(yard_rect, &level_manager);

    loop {
        // Handle events
        if gameplay.update(&mut event_pump) {
            break;
        }


        // Render
        if gameplay.get_state() == YardState::Won {
            println!("won!");
        } else if gameplay.get_state() == YardState::Crashed {
            println!("crashed!");
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        gameplay.render(&mut canvas, &mut game_sprites)?;

        canvas.present();

        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }

    Ok(())
}
