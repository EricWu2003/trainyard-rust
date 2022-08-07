use macroquad::prelude::*;
use crate::GameState;
use crate::gameplay::Gameplay;
use crate::levels::LevelManager;
use crate::{gui::button::Button, sprites::GameSprites, utils::mouse_in_rect};
use crate::gui::button::BUTTON_WIDTH;
use crate::sprites::SoundType;

use super::button::ButtonStyle;

const LIST_ITEM_HEIGHT: f32 = 40.;

pub struct List{
    buttons: Vec<Button>,
    max_height: f32,
    x: f32,
    y:f32,
    initial_index: f32,
    level_manager: LevelManager,
}


impl List {
    pub fn new(x:f32, y:f32, max_height:f32, level_manager: LevelManager) -> List {
        let mut buttons = vec![];
        for city_name in level_manager.get_city_names() {
            buttons.push(Button::new(
                &city_name,
                ButtonStyle::Label,
            ));
            for level_name in level_manager.get_names_in_city(&city_name) {
                buttons.push(Button::new(
                    &level_name,
                    ButtonStyle::LevelNotStarted,
                ));
            }
        }
        
        List {
            buttons,
            x,
            y,
            max_height,
            initial_index: 0.,
            level_manager,
        }
    }

    pub fn render(&self, gs: &GameSprites) {
        let num_to_display = (self.max_height / LIST_ITEM_HEIGHT) as usize;

        let i = self.initial_index as usize;
        for (index, button) in self.buttons[i..i+num_to_display].iter().enumerate() {
            button.render(self.x, self.y + index as f32 * LIST_ITEM_HEIGHT, LIST_ITEM_HEIGHT, gs);
        }
    }

    pub fn push_button(&mut self, button: Button) {
        self.buttons.push(
            button
        )
    }

    pub fn set_max_height(&mut self, new_height: f32) {
        self.max_height = new_height;
    }
    pub fn change_initial_index(&mut self, diff: f32) {
        let mut new_init_index = self.initial_index + diff;
        let num_to_display = (self.max_height / LIST_ITEM_HEIGHT) as i32;
        if new_init_index as i32 + num_to_display > self.buttons.len() as i32 {
            new_init_index = self.buttons.len() as f32 - num_to_display as f32;
        }
        if new_init_index < 0. {
            new_init_index = 0.;
        }
        self.initial_index = new_init_index;
    }

    pub fn update(&mut self, gs: &mut GameSprites, game_state: &mut GameState, gameplay: &mut Gameplay) {
        if is_key_pressed(KeyCode::Up) {
            self.change_initial_index(-1.);
        }
        if is_key_pressed(KeyCode::Down) {
            self.change_initial_index(1.);
        }
        
        if mouse_in_rect(Rect::new(self.x, self.y, BUTTON_WIDTH, self.max_height)) {
            let scroll_amt = -mouse_wheel().1;
            self.change_initial_index(scroll_amt * 0.1);

            if is_mouse_button_pressed(MouseButton::Left) {
                let (_, mut y) = mouse_position();
                y = y - self.y;

                let num_to_display = (self.max_height / LIST_ITEM_HEIGHT) as usize;
                let index = (y/LIST_ITEM_HEIGHT) as usize;
                if index < num_to_display {
                    println!("{}", index);
                    let level_label = self.buttons[self.initial_index as usize + index].label_text.clone();
                    println!("{}", level_label);
                    gs.add_sound(SoundType::ButtonPress);
                    *game_state = GameState::Level;
                    gameplay.reset_yard_from_level(
                        self.level_manager.get_level(&level_label),
                        gs,
                    );
                }
            }
        }
    }

}