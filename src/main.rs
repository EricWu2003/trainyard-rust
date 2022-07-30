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

    let yard_rect = Rect::new(100., 100., 336., 336.);
   
    let mut gameplay = Gameplay::new(yard_rect, &level_manager, &gs);

    loop {
        clear_background(LIGHTGRAY);
        if gameplay.update(&mut gs) {
            break;
        }
        gs.play_sounds();
        gameplay.render(&gs);

        next_frame().await;
    }
}
