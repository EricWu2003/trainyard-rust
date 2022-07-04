use crate::color::Color;

pub struct Edge {
    pub train_to_a: Option<Color>,
    pub train_to_b: Option<Color>,
}

impl Edge {
    pub fn new() -> Edge {
        Edge { train_to_a: None, train_to_b: None }
    }
    
    pub fn interact_trains(&mut self) {
        if let Some(t1) = self.train_to_a {
            if let Some(t2) = self.train_to_b {
                let new_color = Color::mix_many(vec![t1, t2]);
                self.train_to_a = Some(new_color);
                self.train_to_b = Some(new_color);
            }
        }
    }
}
