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
        })
    }
}
