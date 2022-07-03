#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
    fn mix_with(self: Color, other: Color) -> Color {
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
    fn mix_many(trains: &[Color]) -> Color {
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
}
