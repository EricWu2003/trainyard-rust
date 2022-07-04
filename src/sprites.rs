use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub static BYTES_TRACKTILE_BLANK: &[u8; 1891] =
    include_bytes!("../assets/sprites/Tracktile_blank.png");

pub struct GameSprites<'a> {
    pub tracktile_blank: Texture<'a>,
}

impl<'a> GameSprites<'a> {
    pub fn new(
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<GameSprites<'a>, String> {
        Ok(GameSprites {
            tracktile_blank: texture_creator.load_texture_bytes(BYTES_TRACKTILE_BLANK)?,
        })
    }
}
