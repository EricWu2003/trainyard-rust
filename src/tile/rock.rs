use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rock {
    #[serde(skip)]
    pub rect: Option<Rect>
}

impl Rock {
    pub fn new() -> Rock {
        Rock{rect: None}
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = Some(rect);
    }
}