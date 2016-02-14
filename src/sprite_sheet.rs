use std::collections::HashMap;
use sfml::graphics::Sprite;

const SPRITESHEET_PATH: &'static str = "res/sprites";

pub struct SpriteSheet<'a> {
    sprite_map: HashMap<&'static str, Sprite<'a>>
}

impl<'a> SpriteSheet<'a> {
    pub fn new() -> SpriteSheet<'a> {
        SpriteSheet {
            sprite_map: SpriteSheet::generate_map()
        }
    }
    
    pub fn generate_map() -> HashMap<&'static str, Sprite<'a>> {
        return HashMap::new()
    }
}

