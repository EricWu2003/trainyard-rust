use crate::color::Color;
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use soloud::{AudioExt, LoadExt, Soloud};
use soloud::audio::Wav;


pub struct GameSprites<'a> {
    pub atlas: Texture<'a>,
    pub atlas_color: Texture<'a>,
    
    pub tracktile_blank: Rect,
    pub tracktile_b: Rect,
    pub tracktile_h: Rect,
    pub tracktile_jb: Rect,
    pub tracktile_js: Rect,
    pub tracktile_m: Rect,
    pub tracktile_s: Rect,
    pub tracktile_z: Rect,
    pub train: Rect,
    pub plus_sign: Rect,
    pub circle: Rect,
    pub trainsink_entry: Rect,
    pub source_sink_border: Rect,
    pub trainsource_exit: Rect,
    pub rock: Rect,
    pub painter_bg: Rect,
    pub painter_brush: Rect,
    pub splitter_bg: Rect,
    pub splitter: Rect,
    pub sink_satisfied: Rect,
    pub btn_back_to_drawing: Rect,
    pub btn_erase: Rect,
    pub btn_speed: Rect,
    pub btn_start_trains: Rect,
    pub btn_status_crashed: Rect,
    pub btn_status_good: Rect,
    pub btn_stop_erase: Rect,
    pub space_for_speed_slider: Rect,
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
            atlas: texture_creator.load_texture_bytes(include_bytes!("../assets/atlas.png"))?,
            atlas_color: texture_creator.load_texture_bytes(include_bytes!("../assets/atlas.png"))?,
            tracktile_blank: Rect::new(0, 0, 96, 96),
            tracktile_b: Rect::new(96, 0, 96, 96),
            tracktile_h: Rect::new(192, 0, 96, 96),
            tracktile_jb: Rect::new(288, 0, 96, 96),
            tracktile_js: Rect::new(384, 0, 96, 96),
            tracktile_m: Rect::new(576, 96, 96, 96),
            tracktile_s: Rect::new(480, 0, 96, 96),
            tracktile_z: Rect::new(576, 0, 96, 96),
            train: Rect::new(104, 96, 32,57),
            plus_sign: Rect::new(52, 96, 52, 52),
            circle: Rect::new(0, 96, 52,52),
            trainsink_entry: Rect::new(1344, 0, 96, 96),
            source_sink_border: Rect::new(1056, 0, 96,96),
            trainsource_exit: Rect::new(1440, 0, 96,96),
            rock: Rect::new(864, 0, 96, 96),
            painter_bg: Rect::new(672, 0, 96,96),
            painter_brush: Rect::new(768, 0, 96,96),
            splitter_bg: Rect::new(1152, 0, 96, 96),
            splitter: Rect::new(1248, 0, 96, 96),
            sink_satisfied: Rect::new(960, 0, 96,96),
            btn_back_to_drawing: Rect::new(0,1700,424,104),
            btn_erase: Rect::new(0,1908, 208,88),
            btn_speed: Rect::new(424, 1632, 136, 68),
            btn_start_trains: Rect::new(0,1804,424,104),
            btn_status_crashed: Rect::new(424, 1700, 208,168),
            btn_status_good: Rect::new(424, 1868, 208,168),
            btn_stop_erase: Rect::new(208,1908, 208,88),
            space_for_speed_slider: Rect::new(0, 1632, 424, 68),
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
            Color::Brown => tint = (120, 85, 59),
            Color::Blue => tint = (42, 80, 197),
            Color::Red => tint = (187, 39, 31),
            Color::Yellow => tint = (234, 234, 101),
            Color::Orange => tint = (233, 159, 56),
            Color::Green => tint = (96, 201, 59),
            Color::Purple => tint = (161, 32, 197),
        }
        let (red, green, blue) = tint;
        self.atlas_color.set_color_mod(red, green, blue);
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
