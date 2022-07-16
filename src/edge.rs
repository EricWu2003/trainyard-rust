use crate::color::Color;
use crate::GameSprites;

pub struct Edge {
    pub train_to_a: Option<Color>,
    pub train_to_b: Option<Color>,
}

impl Edge {
    pub fn new() -> Edge {
        Edge {
            train_to_a: None,
            train_to_b: None,
        }
    }

    pub fn interact_trains(&mut self, gs: &GameSprites) {
        if let (Some(t1), Some(t2)) = (self.train_to_a, self.train_to_b) {
            let new_color = Color::mix_many(vec![t1, t2]);
            self.train_to_a = Some(new_color);
            self.train_to_b = Some(new_color);
            gs.play_train_sound(new_color);
        }
    }

    pub fn get_char(&self) -> char {
        if self.train_to_a.is_some() && self.train_to_b.is_some() {
            return '2';
        }
        if self.train_to_a.is_some() || self.train_to_b.is_some() {
            return '1';
        }
        return '0';
    }
    pub fn is_empty(&self) -> bool {
        self.train_to_a.is_none() && self.train_to_b.is_none()
    }

    pub fn clear_trains(&mut self) {
        self.train_to_a = None;
        self.train_to_b = None;
    }
}
