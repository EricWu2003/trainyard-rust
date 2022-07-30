use macroquad::prelude::*;
use crate::color::Color;

use soloud::{AudioExt, LoadExt, Soloud};
use soloud::audio::Wav;

use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


// The purpose of this module is to manage which sounds need to get played when a yard is in the Playing state.
// For example, if 4 yellow trains enter 4 different sinks at the same time, we only need to play the sound effect once.
// This prevents lag.
#[derive(PartialEq, Eq, Hash, EnumIter)]
pub enum SoundType {
    ButtonPress,
    TrainBrown,
    TrainYellow,
    TrainRed,
    TrainBlue,
    TrainPurple,
    TrainOrange,
    TrainGreen,
    Splitter,
    Painter,
    DrawTrack,
    SwitchTrack,
    EraseTrack,
    Crash,
    WinLevel,
}

impl SoundType {
    pub fn get_sl_sound<'a> (&'a self, gs: &'a GameSprites) -> &Wav {
        match self {
            Self:: ButtonPress => &gs.sl_button_press,
            Self::TrainBrown => &gs.sl_train_brown,
            Self::TrainYellow => &gs.sl_train_yellow,
            Self::TrainRed => &gs.sl_train_red,
            Self::TrainBlue => &gs.sl_train_blue,
            Self::TrainPurple => &gs.sl_train_purple,
            Self::TrainOrange => &gs.sl_train_orange,
            Self::TrainGreen => &gs.sl_train_green,
            Self::Splitter => &gs.sl_splitter,
            Self::Painter => &gs.sl_painter,
            Self::DrawTrack => &gs.sl_draw_track,
            Self::SwitchTrack => &gs.sl_switch_track,
            Self::EraseTrack => &gs.sl_erase_track,
            Self::Crash => &gs.sl_crash,
            Self::WinLevel => &gs.sl_win_level,
        }
    }
}


pub struct GameSprites {
    // pub atlas: Texture2D,
    // pub atlas_color: Texture2D,
    pub tracktile_blank: Texture2D,
    pub tracktile_b: Texture2D,
    pub tracktile_h: Texture2D,
    pub tracktile_jb: Texture2D,
    pub tracktile_js: Texture2D,
    pub tracktile_m: Texture2D,
    pub tracktile_s: Texture2D,
    pub tracktile_z: Texture2D,
    pub train: Texture2D,
    pub plus_sign: Texture2D,
    pub circle: Texture2D,
    pub trainsink_entry: Texture2D,
    pub source_sink_border: Texture2D,
    pub trainsource_exit: Texture2D,
    pub rock: Texture2D,
    pub painter_bg: Texture2D,
    pub painter_brush: Texture2D,
    pub splitter_bg: Texture2D,
    pub splitter: Texture2D,
    pub sink_satisfied: Texture2D,
    pub btn_back_to_drawing: Texture2D,
    pub btn_erase: Texture2D,
    pub btn_speed: Texture2D,
    pub btn_start_trains: Texture2D,
    pub btn_status_crashed: Texture2D,
    pub btn_status_good: Texture2D,
    pub btn_stop_erase: Texture2D,
    pub space_for_speed_slider: Texture2D,
    pub draw_track_arrow: Texture2D,
    pub smoke: Texture2D,
    pub fire: Texture2D,
    pub fire_small: Texture2D,
    pub star: Texture2D,
    pub star_bright: Texture2D,
    pub painter_brush_animation: Texture2D,
    pub splitter_animation: Texture2D,

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

    pub sounds_to_play: HashMap<SoundType, bool>,
}

fn load_bytes( data: &[u8]) -> Texture2D {
    Texture2D::from_file_with_format(data, None)
}

impl GameSprites {
    pub async fn new() -> GameSprites {
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


        GameSprites {
            // atlas: load_bytes(include_bytes!("../assets/atlas.png")),
            // atlas_color: load_bytes(include_bytes!("../assets/atlas.png")),
            tracktile_blank: load_bytes(include_bytes!("../assets/sprites/Tracktile_blank.png")),
            tracktile_b: load_bytes(include_bytes!("../assets/sprites/Tracktile_b.png")),
            tracktile_h: load_bytes(include_bytes!("../assets/sprites/Tracktile_h.png")),
            tracktile_jb: load_bytes(include_bytes!("../assets/sprites/Tracktile_jb.png")),
            tracktile_js: load_bytes(include_bytes!("../assets/sprites/Tracktile_js.png")),
            tracktile_m: load_bytes(include_bytes!("../assets/sprites/Tracktile_m.png")),
            tracktile_s: load_bytes(include_bytes!("../assets/sprites/Tracktile_s.png")),
            tracktile_z: load_bytes(include_bytes!("../assets/sprites/Tracktile_z.png")),
            train: load_bytes(include_bytes!("../assets/sprites/Train.png")),
            plus_sign: load_bytes(include_bytes!("../assets/sprites/Plus_sign.png")),
            circle: load_bytes(include_bytes!("../assets/sprites/Circle.png")),
            trainsink_entry: load_bytes(include_bytes!("../assets/sprites/Trainsink_entry.png")),
            source_sink_border: load_bytes(include_bytes!("../assets/sprites/Source_sink_border.png")),
            trainsource_exit: load_bytes(include_bytes!("../assets/sprites/Trainsource_exit.png")),
            rock: load_bytes(include_bytes!("../assets/sprites/Rock.png")),
            painter_bg: load_bytes(include_bytes!("../assets/sprites/Painter_bg.png")),
            painter_brush: load_bytes(include_bytes!("../assets/sprites/Painter_brush.png")),
            splitter_bg: load_bytes(include_bytes!("../assets/sprites/Splitter_bg.png")),
            splitter: load_bytes(include_bytes!("../assets/sprites/Splitter.png")),
            sink_satisfied: load_bytes(include_bytes!("../assets/sprites/Sink_satisfied.png")),
            btn_back_to_drawing: load_bytes(include_bytes!("../assets/sprites/Btn_back_to_drawing.png")),
            btn_erase: load_bytes(include_bytes!("../assets/sprites/Btn_erase.png")),
            btn_speed: load_bytes(include_bytes!("../assets/sprites/Btn_speed.png")),
            btn_start_trains: load_bytes(include_bytes!("../assets/sprites/Btn_start_trains.png")),
            btn_status_crashed: load_bytes(include_bytes!("../assets/sprites/Btn_status_crashed.png")),
            btn_status_good: load_bytes(include_bytes!("../assets/sprites/Btn_status_good.png")),
            btn_stop_erase: load_bytes(include_bytes!("../assets/sprites/Btn_stop_erase.png")),
            space_for_speed_slider: load_bytes(include_bytes!("../assets/sprites/Space_for_speed_slider.png")),
            draw_track_arrow: load_bytes(include_bytes!("../assets/sprites/Draw_track_arrow.png")),
            smoke: load_bytes(include_bytes!("../assets/sprites/Smoke.png")),
            fire: load_bytes(include_bytes!("../assets/sprites/Fire.png")),
            fire_small: load_bytes(include_bytes!("../assets/sprites/Fire_small.png")),
            star: load_bytes(include_bytes!("../assets/sprites/Star.png")),
            star_bright: load_bytes(include_bytes!("../assets/sprites/Star_bright.png")),
            painter_brush_animation: load_bytes(include_bytes!("../assets/sprites/Painter_brush_animation.png")),
            splitter_animation: load_bytes(include_bytes!("../assets/sprites/Splitter_animation.png")),

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

            sounds_to_play: HashMap::new(),
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

    pub fn play_train_sound(&mut self, color:Color) {
        match color {
            Color::Brown => self.add_sound(SoundType::TrainBrown),
            Color::Yellow => self.add_sound(SoundType::TrainYellow),
            Color::Blue => self.add_sound(SoundType::TrainBlue),
            Color::Red => self.add_sound(SoundType::TrainRed),
            Color::Orange => self.add_sound(SoundType::TrainOrange),
            Color::Green => self.add_sound(SoundType::TrainGreen),
            Color::Purple => self.add_sound(SoundType::TrainPurple),
        };
    }

    pub fn add_sound(&mut self, sound: SoundType) {
        self.sounds_to_play.insert(sound, true);
    }

    pub fn play_sounds(&mut self) {
        for sound in SoundType::iter() {
            if *self.sounds_to_play.get(&sound).unwrap_or(&false) {
                self.sl.play(sound.get_sl_sound(self));
            }
            self.sounds_to_play.insert(sound, false);
        }
    }
}
