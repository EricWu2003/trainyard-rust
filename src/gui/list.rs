use macroquad::prelude::*;
use crate::{gui::button::Button, sprites::GameSprites};

const LIST_ITEM_HEIGHT: f32 = 40.;
pub enum ListItem {
    Button(Button),
    Label(Button),
}

pub struct List{
    buttons: Vec<Button>,
    max_height: f32,
    x: f32,
    y:f32,
    initial_index: f32,
}


impl List {
    pub fn new(x:f32, y:f32, max_height:f32) -> List {
        List {
            buttons: vec![],
            x,
            y,
            max_height,
            initial_index: 0.,
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

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Up) {
            self.change_initial_index(-1.);
        }
        if is_key_pressed(KeyCode::Down) {
            self.change_initial_index(1.);
        }
        
        let scroll_amt = -mouse_wheel().1;
        self.change_initial_index(scroll_amt * 0.1);
    }

}