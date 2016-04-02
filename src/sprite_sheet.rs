use std::collections::HashMap;
use sfml::graphics::{IntRect, Texture, Sprite};

use level_object::LevelType;
use animation_sprite::AnimationSprite;

const SPRITE_SIZE: f32 = 64.0;
const SPRITESHEET_PATH: &'static str = "res/sprites/game.png";

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum SpriteType {
    PLAYER
}

pub struct SpriteSheet {
    pub texture: Texture,
    sprite_background_map: HashMap<LevelType, IntRect>,
    sprite_foreground_map: HashMap<SpriteType, Vec<IntRect>>
}

impl SpriteSheet {
    pub fn new() -> SpriteSheet {
        SpriteSheet {
            sprite_background_map: SpriteSheet::generate_background_map(),
            sprite_foreground_map: SpriteSheet::generate_foreground_map(),
            texture: Texture::new_from_file(SPRITESHEET_PATH).unwrap()
        }
    }
    
    fn generate_background_map() -> HashMap<LevelType, IntRect> {
        let mut generated_map = HashMap::new();
        generated_map.insert(LevelType::SPACE, IntRect::new((SPRITE_SIZE * 1.0) as i32,
                                                            (SPRITE_SIZE * 0.0) as i32,
                                                            SPRITE_SIZE as i32, SPRITE_SIZE as i32));
        generated_map.insert(LevelType::WALL, IntRect::new((SPRITE_SIZE * 0.0) as i32,
                                                           (SPRITE_SIZE * 0.0) as i32,
                                                            SPRITE_SIZE as i32, SPRITE_SIZE as i32));
        return generated_map;
    }
    
    fn generate_foreground_map() -> HashMap<SpriteType, Vec<IntRect>> {
        let mut generated_map = HashMap::new();
        
        let player_sprites = vec![IntRect::new((SPRITE_SIZE * 0.0) as i32,
                                               (SPRITE_SIZE * 1.0) as i32,
                                                SPRITE_SIZE as i32, SPRITE_SIZE as i32),
                                  IntRect::new((SPRITE_SIZE * 1.0) as i32,
                                               (SPRITE_SIZE * 1.0) as i32,
                                                SPRITE_SIZE as i32, SPRITE_SIZE as i32),
                                  IntRect::new((SPRITE_SIZE * 2.0) as i32,
                                               (SPRITE_SIZE * 1.0) as i32,
                                                SPRITE_SIZE as i32, SPRITE_SIZE as i32)];
        
        generated_map.insert(SpriteType::PLAYER, player_sprites);
        return generated_map;
    }
    
    pub fn generate_background_sprite(&self, level_type: &LevelType) -> Sprite {
        match &self.sprite_background_map.get(level_type) {
            &Some(thing) => {
                let mut sprite = Sprite::new_with_texture(&self.texture).unwrap();
                sprite.set_texture_rect(&thing);
                return sprite;
            },
            &None => panic!("Unable to retrieve LevelType::{:?}. Map's contents was: {:?}", level_type, self.sprite_background_map)
        }
    }
    
    pub fn generate_foreground_sprites(&self, sprite_type: &SpriteType) -> AnimationSprite {
        match &self.sprite_foreground_map.get(sprite_type) {
            &Some(rects) => {
                let mut sprite = Sprite::new_with_texture(&self.texture).unwrap();
                sprite.set_texture_rect(&rects[0]);
                return AnimationSprite::new(sprite, rects.clone());
            }
            &None => panic!("Unable to retrieve SpriteType::{:?}.")
        }
    }
}

