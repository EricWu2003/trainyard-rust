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
pub mod gui;
use crate::gameplay::Gameplay;
use crate::levels::LevelManager;
use crate::sprites::GameSprites;
use crate::gui::list::List;
use macroquad::prelude::*;


pub enum GameState {
    Menu,
    Level(String), // The string represents the level name
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Trainyard".to_owned(),
        window_height: 900,
        window_width: 700,
        ..Default::default()
    }
}

const MARGIN: f32 = 10.;

#[macroquad::main(window_conf)]
async fn main() {
    let level_manager = LevelManager::new();

    let mut gs = GameSprites::new().await;

    let rect = find_yard_rect(screen_height(), screen_width());
   
    let mut gameplay = Gameplay::new(rect, &level_manager, &gs);

    let (mut prev_width, mut prev_height) = (screen_height(), screen_width());


    let mut game_state = GameState::Menu;

    
    let mut list = List::new(MARGIN, MARGIN, screen_height() - MARGIN, level_manager);


    loop {
        clear_background(LIGHTGRAY);
        


        if prev_height != screen_height() || prev_width != screen_width() {
            let rect = find_yard_rect(screen_height(), screen_width());
            gameplay.set_rect(rect, &gs);

            list.set_max_height(screen_height() - MARGIN);

            prev_height = screen_height();
            prev_width = screen_width();
        }

        match game_state {
            GameState::Menu => {
                list.update(&mut gs, &mut game_state, &mut gameplay);
                list.render(&gs);
            },
            GameState::Level(ref level_name) => {
                list.level_manager.set_level_current_progress(&level_name.clone(), &gameplay.get_current_progress());
                list.update_label(level_name.clone());
                if gameplay.update(&mut gs, &mut game_state) {
                    break;
                }


                gameplay.render(&gs);
            },
        }


        gs.play_sounds();
        next_frame().await;
    }
}


fn find_yard_rect(height: f32, width: f32) -> Rect {
    let margin = MARGIN;
    let (height, width) = (height - 2. * margin, width - 2. * margin);

    let aspect_ratio = 874./672.;

    if height < width * aspect_ratio {
        Rect::new(margin, margin, height/aspect_ratio, height)
    } else {
        Rect::new(margin, margin, width, width*aspect_ratio)
    }
}