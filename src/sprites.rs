use crate::color::Color;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub static BYTES_TRACKTILE_BLANK: &[u8; 1891] =
    include_bytes!("../assets/sprites/Tracktile_blank.png");
pub static BYTES_TRACKTILE_B: &[u8; 7157] = include_bytes!("../assets/sprites/Tracktile_b.png");
pub static BYTES_TRACKTILE_H: &[u8; 4624] = include_bytes!("../assets/sprites/Tracktile_h.png");
pub static BYTES_TRACKTILE_JB: &[u8; 7884] = include_bytes!("../assets/sprites/Tracktile_jb.png");
pub static BYTES_TRACKTILE_JB_FLIPPED: &[u8; 8323] =
    include_bytes!("../assets/sprites/Tracktile_jb_flipped.png");
pub static BYTES_TRACKTILE_JS: &[u8; 6515] = include_bytes!("../assets/sprites/Tracktile_js.png");
pub static BYTES_TRACKTILE_JS_FLIPPED: &[u8; 6795] =
    include_bytes!("../assets/sprites/Tracktile_js_flipped.png");
pub static BYTES_TRACKTILE_M: &[u8; 9902] = include_bytes!("../assets/sprites/Tracktile_m.png");
pub static BYTES_TRACKTILE_M_FLIPPED: &[u8; 10282] =
    include_bytes!("../assets/sprites/Tracktile_m_flipped.png");
pub static BYTES_TRACKTILE_S: &[u8; 2918] = include_bytes!("../assets/sprites/Tracktile_s.png");
pub static BYTES_TRACKTILE_Z: &[u8; 10917] = include_bytes!("../assets/sprites/Tracktile_z.png");
pub static BYTES_TRAIN: &[u8; 4943] = include_bytes!("../assets/sprites/Train.png");
pub static BYTES_PLUS_SIGN: &[u8; 1915] = include_bytes!("../assets/sprites/Plus_sign.png");
pub static BYTES_CIRCLE: &[u8; 11531] = include_bytes!("../assets/sprites/Circle.png");
pub static BYTES_TRAINSINK_ENTRY: &[u8; 5046] =
    include_bytes!("../assets/sprites/Trainsink_entry.png");
pub static BYTES_SOURCE_SINK_BORDER: &[u8; 3150] =
    include_bytes!("../assets/sprites/Source_sink_border.png");
pub static BYTES_TRAINSOURCE_EXIT: &[u8; 2900] =
    include_bytes!("../assets/sprites/Trainsource_exit.png");
pub static BYTES_ROCK: &[u8; 16747] = include_bytes!("../assets/sprites/Rock.png");
pub static BYTES_PAINTER_BG: &[u8; 4405] = include_bytes!("../assets/sprites/Painter_bg.png");
pub static BYTES_PAINTER_BRUSH: &[u8; 3689] = include_bytes!("../assets/sprites/Painter_brush.png");
pub static BYTES_SPLITTER_BG: &[u8; 3701] = include_bytes!("../assets/sprites/Splitter_bg.png");
pub static BYTES_SPLITTER: &[u8; 6274] = include_bytes!("../assets/sprites/Splitter.png");
pub static BYTES_SINK_SATISFIED: &[u8; 3426] = include_bytes!("../assets/sprites/Sink_satisfied.png");

pub struct GameSprites<'a> {
    pub tracktile_blank: Texture<'a>,
    pub tracktile_b: Texture<'a>,
    pub tracktile_h: Texture<'a>,
    pub tracktile_jb: Texture<'a>,
    pub tracktile_jb_flipped: Texture<'a>,
    pub tracktile_js: Texture<'a>,
    pub tracktile_js_flipped: Texture<'a>,
    pub tracktile_m: Texture<'a>,
    pub tracktile_m_flipped: Texture<'a>,
    pub tracktile_s: Texture<'a>,
    pub tracktile_z: Texture<'a>,
    pub train: Texture<'a>,
    pub plus_sign: Texture<'a>,
    pub circle: Texture<'a>,
    pub trainsink_entry: Texture<'a>,
    pub source_sink_border: Texture<'a>,
    pub trainsource_exit: Texture<'a>,
    pub rock: Texture<'a>,
    pub painter_bg: Texture<'a>,
    pub painter_brush: Texture<'a>,
    pub splitter_bg: Texture<'a>,
    pub splitter: Texture<'a>,
    pub sink_satisfied: Texture<'a>,
}

impl<'a> GameSprites<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<GameSprites<'a>, String> {
        Ok(GameSprites {
            tracktile_blank: texture_creator.load_texture_bytes(BYTES_TRACKTILE_BLANK)?,
            tracktile_b: texture_creator.load_texture_bytes(BYTES_TRACKTILE_B)?,
            tracktile_h: texture_creator.load_texture_bytes(BYTES_TRACKTILE_H)?,
            tracktile_jb: texture_creator.load_texture_bytes(BYTES_TRACKTILE_JB)?,
            tracktile_jb_flipped: texture_creator.load_texture_bytes(BYTES_TRACKTILE_JB_FLIPPED)?,
            tracktile_js: texture_creator.load_texture_bytes(BYTES_TRACKTILE_JS)?,
            tracktile_js_flipped: texture_creator.load_texture_bytes(BYTES_TRACKTILE_JS_FLIPPED)?,
            tracktile_m: texture_creator.load_texture_bytes(BYTES_TRACKTILE_M)?,
            tracktile_m_flipped: texture_creator.load_texture_bytes(BYTES_TRACKTILE_M_FLIPPED)?,
            tracktile_s: texture_creator.load_texture_bytes(BYTES_TRACKTILE_S)?,
            tracktile_z: texture_creator.load_texture_bytes(BYTES_TRACKTILE_Z)?,
            train: texture_creator.load_texture_bytes(BYTES_TRAIN)?,
            plus_sign: texture_creator.load_texture_bytes(BYTES_PLUS_SIGN)?,
            circle: texture_creator.load_texture_bytes(BYTES_CIRCLE)?,
            trainsink_entry: texture_creator.load_texture_bytes(BYTES_TRAINSINK_ENTRY)?,
            source_sink_border: texture_creator.load_texture_bytes(BYTES_SOURCE_SINK_BORDER)?,
            trainsource_exit: texture_creator.load_texture_bytes(BYTES_TRAINSOURCE_EXIT)?,
            rock: texture_creator.load_texture_bytes(BYTES_ROCK)?,
            painter_bg: texture_creator.load_texture_bytes(BYTES_PAINTER_BG)?,
            painter_brush: texture_creator.load_texture_bytes(BYTES_PAINTER_BRUSH)?,
            splitter_bg: texture_creator.load_texture_bytes(BYTES_SPLITTER_BG)?,
            splitter: texture_creator.load_texture_bytes(BYTES_SPLITTER)?,
            sink_satisfied: texture_creator.load_texture_bytes(BYTES_SINK_SATISFIED)?,
        })
    }

    pub fn set_color(&mut self, color: Color) {
        let tint;
        match color {
            Color::Brown => tint = (139, 69, 19),
            Color::Blue => tint = (0, 0, 255),
            Color::Red => tint = (255, 0, 0),
            Color::Yellow => tint = (255, 255, 0),
            Color::Orange => tint = (255, 140, 0),
            Color::Green => tint = (34, 139, 34),
            Color::Purple => tint = (148, 0, 211),
        }
        let (red, green, blue) = tint;
        self.train.set_color_mod(red, green, blue);
        self.plus_sign.set_color_mod(red, green, blue);
        self.circle.set_color_mod(red, green, blue);
        self.painter_brush.set_color_mod(red, green, blue);
    }
}
