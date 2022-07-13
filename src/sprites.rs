use crate::color::Color;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use soloud::{AudioExt, LoadExt, Soloud};
use soloud::audio::Wav;

pub static BYTES_TRACKTILE_BLANK: &[u8; 1891] =
    include_bytes!("../assets/sprites/Tracktile_blank.png");
pub static BYTES_TRACKTILE_B: &[u8; 7157] = include_bytes!("../assets/sprites/Tracktile_b.png");
pub static BYTES_TRACKTILE_H: &[u8; 4624] = include_bytes!("../assets/sprites/Tracktile_h.png");
pub static BYTES_TRACKTILE_JB: &[u8; 7884] = include_bytes!("../assets/sprites/Tracktile_jb.png");
pub static BYTES_TRACKTILE_JS: &[u8; 6515] = include_bytes!("../assets/sprites/Tracktile_js.png");
pub static BYTES_TRACKTILE_M: &[u8; 9902] = include_bytes!("../assets/sprites/Tracktile_m.png");
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
pub static BYTES_SINK_SATISFIED: &[u8; 3426] =
    include_bytes!("../assets/sprites/Sink_satisfied.png");
pub static BYTES_BTN_BACK_TO_DRAWING: &[u8; 29990] = include_bytes!("../assets/UI/Btn_back_to_drawing.png");
pub static BYTES_BTN_ERASE: &[u8; 17439] = include_bytes!("../assets/UI/Btn_erase.png");
pub static BYTES_BTN_SPEED: &[u8; 9213] = include_bytes!("../assets/UI/Btn_speed.png");
pub static BYTES_BTN_START_TRAINS: &[u8; 25436] = include_bytes!("../assets/UI/Btn_start_trains.png");
pub static BYTES_BTN_STATUS_CRASHED: &[u8; 13777] = include_bytes!("../assets/UI/Btn_status_crashed.png");
pub static BYTES_BTN_STATUS_GOOD: &[u8; 11736] = include_bytes!("../assets/UI/Btn_status_good.png");
pub static BYTES_BTN_STOP_ERASE: &[u8; 19894] = include_bytes!("../assets/UI/Btn_stop_erase.png");
pub static BYTES_SPACE_FOR_SPEED_SLIDER: &[u8; 5034] = include_bytes!("../assets/UI/Space_for_speed_slider.png");

pub struct GameSprites<'a> {
    pub tracktile_blank: Texture<'a>,
    pub tracktile_b: Texture<'a>,
    pub tracktile_h: Texture<'a>,
    pub tracktile_jb: Texture<'a>,
    pub tracktile_js: Texture<'a>,
    pub tracktile_m: Texture<'a>,
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
    pub btn_back_to_drawing: Texture<'a>,
    pub btn_erase: Texture<'a>,
    pub btn_speed: Texture<'a>,
    pub btn_start_trains: Texture<'a>,
    pub btn_status_crashed: Texture<'a>,
    pub btn_status_good: Texture<'a>,
    pub btn_stop_erase: Texture<'a>,
    pub space_for_speed_slider: Texture<'a>,
    pub sl: Soloud,
    pub sl_button_press: Wav,
    pub sl_train_brown: Wav,
    pub sl_train_yellow: Wav,
    pub sl_train_red: Wav,
    pub sl_train_blue: Wav,
    pub sl_train_purple: Wav,
    pub sl_train_orange: Wav,
    pub sl_train_green: Wav,
    pub sl_splitter: Wav,
    pub sl_painter: Wav,
    pub sl_draw_track: Wav,
    pub sl_switch_track: Wav,
    pub sl_erase_track: Wav,
    pub sl_crash: Wav,
    pub sl_win_level: Wav,
}

impl<'a> GameSprites<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<GameSprites<'a>, String> {
        let mut sl_button_press = Wav::default();
        sl_button_press.load_mem(include_bytes!("../assets/audio/button_press.ogg")).unwrap();
        let mut sl_train_brown = Wav::default();
        sl_train_brown.load_mem(include_bytes!("../assets/audio/train_brown.ogg")).unwrap();
        let mut sl_train_yellow = Wav::default();
        sl_train_yellow.load_mem(include_bytes!("../assets/audio/train_yellow.ogg")).unwrap();
        let mut sl_train_red = Wav::default();
        sl_train_red.load_mem(include_bytes!("../assets/audio/train_red.ogg")).unwrap();
        let mut sl_train_blue = Wav::default();
        sl_train_blue.load_mem(include_bytes!("../assets/audio/train_blue.ogg")).unwrap();
        let mut sl_train_purple = Wav::default();
        sl_train_purple.load_mem(include_bytes!("../assets/audio/train_purple.ogg")).unwrap();
        let mut sl_train_orange = Wav::default();
        sl_train_orange.load_mem(include_bytes!("../assets/audio/train_orange.ogg")).unwrap();
        let mut sl_train_green = Wav::default();
        sl_train_green.load_mem(include_bytes!("../assets/audio/train_green.ogg")).unwrap();
        let mut sl_splitter = Wav::default();
        sl_splitter.load_mem(include_bytes!("../assets/audio/splitter.ogg")).unwrap();
        let mut sl_painter = Wav::default();
        sl_painter.load_mem(include_bytes!("../assets/audio/painter.ogg")).unwrap();
        let mut sl_draw_track = Wav::default();
        sl_draw_track.load_mem(include_bytes!("../assets/audio/draw_track.ogg")).unwrap();
        let mut sl_switch_track = Wav::default();
        sl_switch_track.load_mem(include_bytes!("../assets/audio/switch_track.ogg")).unwrap();
        let mut sl_erase_track = Wav::default();
        sl_erase_track.load_mem(include_bytes!("../assets/audio/erase_track.ogg")).unwrap();
        let mut sl_crash = Wav::default();
        sl_crash.load_mem(include_bytes!("../assets/audio/crash.ogg")).unwrap();
        let mut sl_win_level = Wav::default();
        sl_win_level.load_mem(include_bytes!("../assets/audio/win_level.ogg")).unwrap();


        Ok(GameSprites {
            tracktile_blank: texture_creator.load_texture_bytes(BYTES_TRACKTILE_BLANK)?,
            tracktile_b: texture_creator.load_texture_bytes(BYTES_TRACKTILE_B)?,
            tracktile_h: texture_creator.load_texture_bytes(BYTES_TRACKTILE_H)?,
            tracktile_jb: texture_creator.load_texture_bytes(BYTES_TRACKTILE_JB)?,
            tracktile_js: texture_creator.load_texture_bytes(BYTES_TRACKTILE_JS)?,
            tracktile_m: texture_creator.load_texture_bytes(BYTES_TRACKTILE_M)?,
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
            btn_back_to_drawing: texture_creator.load_texture_bytes(BYTES_BTN_BACK_TO_DRAWING)?,
            btn_erase: texture_creator.load_texture_bytes(BYTES_BTN_ERASE)?,
            btn_speed: texture_creator.load_texture_bytes(BYTES_BTN_SPEED)?,
            btn_start_trains: texture_creator.load_texture_bytes(BYTES_BTN_START_TRAINS)?,
            btn_status_crashed: texture_creator.load_texture_bytes(BYTES_BTN_STATUS_CRASHED)?,
            btn_status_good: texture_creator.load_texture_bytes(BYTES_BTN_STATUS_GOOD)?,
            btn_stop_erase: texture_creator.load_texture_bytes(BYTES_BTN_STOP_ERASE)?,
            space_for_speed_slider: texture_creator.load_texture_bytes(BYTES_SPACE_FOR_SPEED_SLIDER)?,
            sl: Soloud::default().unwrap(),
            sl_button_press,
            sl_train_brown,
            sl_train_yellow,
            sl_train_red,
            sl_train_blue,
            sl_train_purple,
            sl_train_orange,
            sl_train_green,
            sl_splitter,
            sl_painter,
            sl_draw_track,
            sl_switch_track,
            sl_erase_track,
            sl_crash,
            sl_win_level,
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

    pub fn play_train_sound(&self, color:Color) {
        match color {
            Color::Brown => self.sl.play(&self.sl_train_brown),
            Color::Yellow => self.sl.play(&self.sl_train_yellow),
            Color::Blue => self.sl.play(&self.sl_train_blue),
            Color::Red => self.sl.play(&self.sl_train_red),
            Color::Orange => self.sl.play(&self.sl_train_orange),
            Color::Green => self.sl.play(&self.sl_train_green),
            Color::Purple => self.sl.play(&self.sl_train_purple),
        };
    }
}
