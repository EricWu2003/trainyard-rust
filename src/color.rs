use macroquad::color::Color as macroColor;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Color {
    Brown,
    Red,
    Blue,
    Yellow,
    Purple,
    Green,
    Orange,
}

impl Color {
    pub fn mix_with(self: Color, other: Color) -> Color {
        if self == other {
            return other;
        }

        if (self == Color::Blue && other == Color::Red)
            || (self == Color::Red && other == Color::Blue)
        {
            return Color::Purple;
        }
        if (self == Color::Yellow && other == Color::Red)
            || (self == Color::Red && other == Color::Yellow)
        {
            return Color::Orange;
        }
        if (self == Color::Yellow && other == Color::Blue)
            || (self == Color::Blue && other == Color::Yellow)
        {
            return Color::Green;
        }

        Color::Brown
    }
    pub fn mix_many(trains: Vec<Color>) -> Color {
        match trains.len() {
            1 => trains[0],
            2 => trains[1].mix_with(trains[0]),
            _ => {
                for i in 0..(trains.len() - 1) {
                    if trains[i] != trains[i + 1] {
                        return Color::Brown;
                    }
                }
                trains[0]
            }
        }
    }

    pub fn get_color(&self) -> macroColor {
        match self {
            Color:: Brown => macroColor::new(0.471, 0.333, 0.231, 1.),
            Color::Blue => macroColor::new(0.165, 0.314, 0.773, 1.),
            Color::Red => macroColor::new(0.733, 0.153, 0.122, 1.),
            Color::Yellow => macroColor::new(0.918, 0.918, 0.396, 1.),
            Color::Orange => macroColor::new(0.914, 0.624, 0.220, 1.),
            Color::Green => macroColor::new(0.376, 0.788, 0.231, 1.),
            Color::Purple => macroColor::new(0.631, 0.125, 0.773, 1.),
        }
    }
}
