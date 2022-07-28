use macroquad::prelude::*;
use macroquad::audio::Sound;
use macroquad::audio::load_sound_from_bytes as load_sound;
use macroquad::audio::play_sound_once;
use crate::color::Color;

// use soloud::{AudioExt, LoadExt, Soloud};
// use soloud::audio::Wav;


pub struct GameSprites {
    pub atlas: Texture2D,
    pub atlas_color: Texture2D,
    
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
    pub draw_track_arrow: Rect,
    pub smoke: Rect,
    pub fire: Rect,
    pub fire_small: Rect,
    pub star: Rect,
    pub star_bright: Rect,
    pub sl_button_press: Sound,
    pub sl_train_brown: Sound,
    pub sl_train_yellow: Sound,
    pub sl_train_red: Sound,
    pub sl_train_blue: Sound,
    pub sl_train_purple: Sound,
    pub sl_train_orange: Sound,
    pub sl_train_green: Sound,
    pub sl_splitter: Sound,
    pub sl_painter: Sound,
    pub sl_draw_track: Sound,
    pub sl_switch_track: Sound,
    pub sl_erase_track: Sound,
    pub sl_crash: Sound,
    pub sl_win_level: Sound,
}

impl GameSprites {
    pub async fn new() -> GameSprites {
        let sl_button_press = 
            load_sound(include_bytes!("../assets/audio/button_press.ogg")).await.unwrap();
        let sl_train_brown = 
            load_sound(include_bytes!("../assets/audio/train_brown.ogg")).await.unwrap();
        let sl_train_yellow = 
            load_sound(include_bytes!("../assets/audio/train_yellow.ogg")).await.unwrap();
        let sl_train_red = 
            load_sound(include_bytes!("../assets/audio/train_red.ogg")).await.unwrap();
        let sl_train_blue = 
            load_sound(include_bytes!("../assets/audio/train_blue.ogg")).await.unwrap();
        let sl_train_purple = 
            load_sound(include_bytes!("../assets/audio/train_purple.ogg")).await.unwrap();
        let sl_train_orange = 
            load_sound(include_bytes!("../assets/audio/train_orange.ogg")).await.unwrap();
        let sl_train_green = 
            load_sound(include_bytes!("../assets/audio/train_green.ogg")).await.unwrap();
        let sl_splitter = 
            load_sound(include_bytes!("../assets/audio/splitter.ogg")).await.unwrap();
        let sl_painter = 
            load_sound(include_bytes!("../assets/audio/painter.ogg")).await.unwrap();
        let sl_draw_track = 
            load_sound(include_bytes!("../assets/audio/draw_track.ogg")).await.unwrap();
        let sl_switch_track = 
            load_sound(include_bytes!("../assets/audio/switch_track.ogg")).await.unwrap();
        let sl_erase_track = 
            load_sound(include_bytes!("../assets/audio/erase_track.ogg")).await.unwrap();
        let sl_crash = 
            load_sound(include_bytes!("../assets/audio/crash.ogg")).await.unwrap();
        let sl_win_level = 
            load_sound(include_bytes!("../assets/audio/win_level.ogg")).await.unwrap();


        GameSprites {
            atlas: Texture2D::from_file_with_format(include_bytes!("../assets/atlas.png"), None),
            atlas_color: Texture2D::from_file_with_format(include_bytes!("../assets/atlas.png"), None),
            tracktile_blank: Rect::new(0., 0., 96., 96.),
            tracktile_b: Rect::new(96., 0., 96., 96.),
            tracktile_h: Rect::new(192., 0., 96., 96.),
            tracktile_jb: Rect::new(288., 0., 96., 96.),
            tracktile_js: Rect::new(384., 0., 96., 96.),
            tracktile_m: Rect::new(576., 96., 96., 96.),
            tracktile_s: Rect::new(480., 0., 96., 96.),
            tracktile_z: Rect::new(576., 0., 96., 96.),
            train: Rect::new(104., 96., 32.,57.),
            plus_sign: Rect::new(52., 96., 52., 52.),
            circle: Rect::new(0., 96., 52.,52.),
            trainsink_entry: Rect::new(1344., 0., 96., 96.),
            source_sink_border: Rect::new(1056., 0., 96.,96.),
            trainsource_exit: Rect::new(1440., 0., 96.,96.),
            rock: Rect::new(864., 0., 96., 96.),
            painter_bg: Rect::new(672., 0., 96.,96.),
            painter_brush: Rect::new(768., 0., 96.,96.),
            splitter_bg: Rect::new(1152., 0., 96., 96.),
            splitter: Rect::new(1248., 0., 96., 96.),
            sink_satisfied: Rect::new(960., 0., 96.,96.),
            btn_back_to_drawing: Rect::new(0.,1700.,424.,104.),
            btn_erase: Rect::new(0.,1908., 208.,88.),
            btn_speed: Rect::new(424., 1632., 136., 68.),
            btn_start_trains: Rect::new(0.,1804.,424.,104.),
            btn_status_crashed: Rect::new(424., 1700., 208.,168.),
            btn_status_good: Rect::new(424., 1868., 208.,168.),
            btn_stop_erase: Rect::new(208.,1908., 208.,88.),
            space_for_speed_slider: Rect::new(0., 1632., 424., 68.),
            draw_track_arrow: Rect::new(136., 96., 36., 36.),
            smoke: Rect::new(172., 96., 64., 64.),
            fire: Rect::new(236., 96., 64., 64.),
            fire_small: Rect::new(300., 96., 32., 32.),
            star: Rect::new(300., 128., 32., 32.),
            star_bright: Rect::new(332., 96., 32., 32.),
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
        }
    }

    // pub fn set_color(&mut self, color: Color) {
    //     let tint;
    //     match color {
    //         Color::Brown => tint = (120, 85, 59),
    //         Color::Blue => tint = (42, 80, 197),
    //         Color::Red => tint = (187, 39, 31),
    //         Color::Yellow => tint = (234, 234, 101),
    //         Color::Orange => tint = (233, 159, 56),
    //         Color::Green => tint = (96, 201, 59),
    //         Color::Purple => tint = (161, 32, 197),
    //     }
    //     let (red, green, blue) = tint;
    //     self.atlas_color.set_color_mod(red, green, blue);
    // }
    // pub fn set_alpha(&mut self, alpha: u8) {
    //     self.atlas_color.set_alpha_mod(alpha);
    // }

    pub fn play_train_sound(&self, color:Color) {
        match color {
            Color::Brown => play_sound_once(self.sl_train_brown),
            Color::Yellow => play_sound_once(self.sl_train_yellow),
            Color::Blue => play_sound_once(self.sl_train_blue),
            Color::Red => play_sound_once(self.sl_train_red),
            Color::Orange => play_sound_once(self.sl_train_orange),
            Color::Green => play_sound_once(self.sl_train_green),
            Color::Purple => play_sound_once(self.sl_train_purple),
        };
    }
}
