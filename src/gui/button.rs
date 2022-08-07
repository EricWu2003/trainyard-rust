use macroquad::prelude::*;

use crate::sprites::GameSprites;


// const BUTTON_HEIGHT: f32 = 20.;
const BUTTON_WIDTH: f32 = 350.;
const BUTTON_MARGIN_LEFT: f32 = 8.;
const BUTTON_COLOR: Color = WHITE;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum ButtonStyle {
    Label,
    Tutorial,
    LevelNotStarted,
    LevelInProgress,
    LevelSolved,
}
pub struct Button {
    pub label_text: String,
    pub style: ButtonStyle,
}

impl Button {
    pub fn new(label_text: &str, style: ButtonStyle) -> Button {
        Button {
            label_text: label_text.to_owned(),
            style,
        }
    }

    pub fn render(&self, x: f32, y: f32, height: f32, gs: &GameSprites) {
        draw_rectangle(x, y, BUTTON_WIDTH, height, BUTTON_COLOR);
        draw_rectangle_lines(x, y, BUTTON_WIDTH, height, 1., BLACK);
        let font_size = height * 0.7;

        let TextDimensions {height: text_height, width, ..} = measure_text(&self.label_text, None, font_size as u16, 1.);

        let color = if width < BUTTON_WIDTH - BUTTON_MARGIN_LEFT {DARKGRAY} else {RED};

        let mut params = TextParams {
            font_size: font_size as u16,
            color,
            ..Default::default()
        };
        if self.style == ButtonStyle::Label {
            params.font = gs.label_font;
            params.color = DARKBLUE;
        }

        draw_text_ex(
            &self.label_text,
            x + BUTTON_MARGIN_LEFT,
            y + height - (height - text_height)/2.,
            params,
        );
    }
}