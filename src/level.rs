use sfml::graphics::{RenderTarget, Color};
use sfml::traits::Drawable;

use level_object::{LevelObject, LevelType};
use sprite_sheet::SpriteSheet;

pub struct Level<'s> {
    width: usize,
    height: usize,
    map: Vec<Vec<LevelObject<'s>>>,
    sprite_sheet: SpriteSheet<'s>,
}

impl<'s> Level<'s> {
    pub fn new(width: u32, height: u32) -> Level<'s> {
        Level {
            width: width as usize,
            height: height as usize,
            map: vec![vec![]],
            sprite_sheet: SpriteSheet::new()
        }
    }
    
    pub fn new_with_map(map: Vec<Vec<LevelType>>) -> Level<'s> {
        let new_map = Level::setup_map(&map);
        println!("{} {}", new_map.len(), new_map[0].len());
        Level {
            width: map.len(),
            height: map[0].len(),
            map: new_map,
            sprite_sheet: SpriteSheet::new()
        }
    }
    
    pub fn setup_map(map: &Vec<Vec<LevelType>>) -> Vec<Vec<LevelObject<'s>>> {

        let mut returned_map = vec![vec![]];
        
        for x in 0..map.len() {
            returned_map.push(Vec::<LevelObject>::new());
            for y in 0..map[0].len() {
                let level_type = map[x][y].clone();
                let mut level_object = LevelObject::new(level_type);
                
                let new_fill_colour = &match level_object.level_type {
                    LevelType::SPACE => Color::red(),
                    LevelType::WALL => Color::blue()
                };
                
                level_object.sprite.set_fill_color(new_fill_colour);
                level_object.sprite.set_position2f((x as u32 * super::GAME_SIZE) as f32,
                                                   (y as u32 * super::GAME_SIZE) as f32);
                level_object.sprite.set_size2f(super::GAME_SIZE as f32,
                                               super::GAME_SIZE as f32);
                
                returned_map[x].push(level_object);
            }
        }
        return returned_map;
    }
}

impl<'s> Drawable for Level<'s> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.map[x][y].draw(target);
            }
        }
    }
}