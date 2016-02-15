use sfml::system::{Vector2f};
use sfml::graphics::{RenderStates, RenderTarget, Color, FloatRect, Vertex, VertexArray, PrimitiveType};
use sfml::traits::Drawable;

use level_object::{LevelObject, LevelType};
use sprite_sheet::SpriteSheet;

pub struct Level {
    width: usize,
    height: usize,
    map: Vec<Vec<LevelObject>>,
    sprite_sheet: SpriteSheet,
}

impl Level {
/*
    pub fn new(width: u32, height: u32) -> Level {
        Level {
            width: width as usize,
            height: height as usize,
            map: vec![vec![]],
            sprite_sheet: SpriteSheet::new()
        }
    }
 */   
    pub fn new_with_map(map: Vec<Vec<LevelType>>) -> Level {
        let new_map = Level::setup_map(&map);
        println!("{} {}", new_map.len(), new_map[0].len());
        Level {
            width: map.len(),
            height: map[0].len(),
            map: new_map,
            sprite_sheet: SpriteSheet::new()
        }
    }
    
    pub fn setup_map(map: &Vec<Vec<LevelType>>) -> Vec<Vec<LevelObject>> {

        let mut returned_map = vec![vec![]];
        
        for x in 0..map.len() {
            returned_map.push(Vec::<LevelObject>::new());
            for y in 0..map[0].len() {
                let level_type = map[x][y].clone();
                let mut level_object = LevelObject::new(level_type);

                level_object.position = FloatRect::new((x as u32 * super::GAME_SIZE) as f32,
                                                       (y as u32 * super::GAME_SIZE) as f32,
                                                       super::GAME_SIZE as f32,
                                                       super::GAME_SIZE as f32);
                
                returned_map[x].push(level_object);
            }
        }
        return returned_map;
    }
}

impl Drawable for Level {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        let mut vertex_array = VertexArray::new_init(PrimitiveType::Quads, (self.width * self.height * 4) as u32).unwrap();
        let mut vertex_count = 0;
        
        for x in 0..self.width {
            for y in 0..self.height {
                let level_object = &self.map[x][y];
                let texture_rect = self.sprite_sheet.get_texture_rect(&level_object.level_type);
                
                // Bottom left
                vertex_array.get_vertex(vertex_count + 0).position = Vector2f::new(level_object.position.left,
                                                                                   level_object.position.top + level_object.position.height);
                vertex_array.get_vertex(vertex_count + 0).tex_coords = Vector2f::new(texture_rect.left,
                                                                                   texture_rect.top + texture_rect.height);
                
                // Top left
                vertex_array.get_vertex(vertex_count + 1).position = Vector2f::new(level_object.position.left,
                                                                                   level_object.position.top);
                vertex_array.get_vertex(vertex_count + 1).tex_coords = Vector2f::new(texture_rect.left,
                                                                                     texture_rect.top);
                
                // Top right
                vertex_array.get_vertex(vertex_count + 2).position = Vector2f::new(level_object.position.left +
                                                                                   level_object.position.width, level_object.position.top);
                vertex_array.get_vertex(vertex_count + 2).tex_coords = Vector2f::new(texture_rect.left +
                                                                                     texture_rect.width, texture_rect.top);
                
                // Bottom right
                vertex_array.get_vertex(vertex_count + 3).position = Vector2f::new(level_object.position.left +
                                                                                   level_object.position.width, level_object.position.top + level_object.position.height);
                vertex_array.get_vertex(vertex_count + 3).tex_coords = Vector2f::new(texture_rect.left +
                                                                                    texture_rect.width, texture_rect.top + texture_rect.height);
                vertex_count += 4;
            }
        }
        let mut states = RenderStates::default();
        states.texture = Some(&self.sprite_sheet.texture);
        target.draw_with_renderstates(&vertex_array, &mut states);
    }
}