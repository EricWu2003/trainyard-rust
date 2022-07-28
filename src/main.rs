pub mod color;
// pub mod connection;
// pub mod edge;
// pub mod gameplay;
// pub mod levels;
pub mod sprites;
// pub mod tile;
// pub mod yard;
// pub mod utils;
// pub mod particle;
// use crate::gameplay::Gameplay;
// use crate::levels::LevelManager;
use crate::sprites::GameSprites;
use macroquad::prelude::*;

#[macroquad::main("Trainyard")]
async fn main() {
    // let level_manager = LevelManager::new();

    let gs = GameSprites::new().await;

   
    // let mut gameplay = Gameplay::new(yard_rect, &level_manager);

    loop {
        clear_background(LIGHTGRAY);
        draw_texture_ex(
           gs.atlas,
            screen_width() / 2.,
            screen_height() / 2.,
            BLUE,
            DrawTextureParams { dest_size: None, source: Some(gs.train), rotation: 0., flip_x: false, flip_y: false, pivot: None }
        );
        next_frame().await;
    }
}
