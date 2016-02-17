use std::collections::HashMap;
use sfml::graphics::{FloatRect, Texture};

use level_object::LevelType;

const SPRITE_SIZE: f32 = 64.0;
const SPRITESHEET_PATH: &'static str = "res/sprites/game.png";

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum SpriteType {
    PLAYER
}

pub struct SpriteSheet {
    pub texture: Texture,
    sprite_background_map: HashMap<LevelType, FloatRect>,
    sprite_foreground_map: HashMap<SpriteType, FloatRect>
}

impl SpriteSheet {
    pub fn new() -> SpriteSheet {
        SpriteSheet {
            sprite_background_map: SpriteSheet::generate_background_map(),
            sprite_foreground_map: SpriteSheet::generate_foreground_map(),
            texture: Texture::new_from_file(SPRITESHEET_PATH).unwrap()
        }
    }
    
    fn generate_background_map() -> HashMap<LevelType, FloatRect> {
        let mut generated_map = HashMap::new();
        generated_map.insert(LevelType::SPACE, FloatRect::new(SPRITE_SIZE * 1.0, SPRITE_SIZE * 0.0, SPRITE_SIZE, SPRITE_SIZE));
        generated_map.insert(LevelType::WALL, FloatRect::new(SPRITE_SIZE * 0.0, SPRITE_SIZE * 0.0, SPRITE_SIZE, SPRITE_SIZE));
        return generated_map;
    }
    
    fn generate_foreground_map() -> HashMap<SpriteType, FloatRect> {
        let mut generated_map = HashMap::new();
        generated_map.insert(SpriteType::PLAYER, FloatRect::new(SPRITE_SIZE * 0.0,  SPRITE_SIZE * 1.0, SPRITE_SIZE, SPRITE_SIZE));
        return generated_map;
    }
    
    pub fn get_background_texture_rect(&self, level_type: &LevelType) -> &FloatRect {
        match &self.sprite_background_map.get(level_type) {
            &Some(thing) => thing,
            &None => panic!("Unable to retrieve LevelType::{:?}. Map's contents was: {:?}", level_type, self.sprite_background_map)
        }
    }
    
    pub fn get_foreground_texture_rect(&self, sprite_type: &SpriteType) -> &FloatRect {
        match &self.sprite_foreground_map.get(sprite_type) {
            &Some(thing) => thing,
            &None => panic!("Unable to retrieve SpriteType::{:?}.")
        }
    }
}

