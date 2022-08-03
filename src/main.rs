pub mod color;
pub mod connection;
pub mod edge;
pub mod gameplay;
pub mod levels;
pub mod sprites;
pub mod tile;
pub mod yard;
pub mod utils;
pub mod particle;
use crate::gameplay::Gameplay;
use crate::levels::LevelManager;
use crate::sprites::GameSprites;
use macroquad::prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "Trainyard".to_owned(),
        window_height: 900,
        window_width: 700,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let level_manager = LevelManager::new();

    let mut gs = GameSprites::new().await;

    let rect = find_yard_rect(screen_height(), screen_width());
   
    let mut gameplay = Gameplay::new(rect, &level_manager, &gs);

    let (mut prev_width, mut prev_height) = (screen_height(), screen_width());

    loop {
        clear_background(LIGHTGRAY);
        if gameplay.update(&mut gs) {
            break;
        }
        gs.play_sounds();
        gameplay.render(&gs);


        if prev_height != screen_height() || prev_width != screen_width() {
            let rect = find_yard_rect(screen_height(), screen_width());
            gameplay.set_rect(rect, &gs);

            prev_height = screen_height();
            prev_width = screen_width();
        }


        next_frame().await;
    }
}


fn find_yard_rect(height: f32, width: f32) -> Rect {
    let margin = 10.;
    let (height, width) = (height - 2. * margin, width - 2. * margin);

    let aspect_ratio = 874./672.;

    if height < width * aspect_ratio {
        Rect::new(margin, margin, height/aspect_ratio, height)
    } else {
        Rect::new(margin, margin, width, width*aspect_ratio)
    }
}