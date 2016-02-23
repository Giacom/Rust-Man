use std::cmp;

use sfml::system::{Vector2f, Vector2u};
use sfml::graphics::{Color, RenderStates, RenderTarget, FloatRect, VertexArray, PrimitiveType, Image};
use sfml::traits::Drawable;

use level_object::{LevelObject, LevelType};
use sprite_sheet::SpriteSheet;

pub struct Level<'a> {
    pub size: Vector2u,
    map: Vec<Vec<LevelObject>>,
    sprite_sheet: &'a SpriteSheet,
    vertex_array: VertexArray
}

impl<'a> Level<'a> {
 
    pub fn new_with_map(size: Vector2u, map: Vec<Vec<LevelType>>, sprite_sheet: &'a SpriteSheet) -> Level<'a> {
        let new_map = Level::setup_map(&map);

        Level {
            size: size,
            map: new_map,
            sprite_sheet: sprite_sheet,
            vertex_array: VertexArray::new_init(PrimitiveType::Quads, (size.x * size.y * 4)).unwrap()
        }
    }
    
    pub fn new_with_image(image: &Image, sprite_sheet: &'a SpriteSheet) -> Level<'a> {
        let level_size = image.get_size();
        let mut map = vec![];
        
        for x in 0..level_size.x + 1 {
            map.push(vec![]);
            for y in 0..level_size.y + 1 {
                let level_type = match image.get_pixel(x, y) {
                    Color { red: 0, green: 0, blue: 255, alpha: 255 } => { LevelType::WALL },
                    _ => { LevelType::SPACE }
                };
                map.last_mut().unwrap().push(level_type);
            }
        }
        
        Level::new_with_map(level_size, map, sprite_sheet)
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
    
    pub fn get_tile(&self, x: i32, y: i32) -> &LevelObject {
       return &self.map[cmp::max(cmp::min(x, (self.size.x - 1) as i32), 0) as usize][cmp::max(cmp::min(y, (self.size.y - 1) as i32), 0) as usize];
    }
    
    pub fn world_to_tile(x: f32, y: f32) -> (i32, i32) {
        let mut x = x as i32;
        let mut y = y as i32;
        
        x &= !(super::GAME_SIZE as i32 - 1);
        y &= !(super::GAME_SIZE as i32 - 1);
        if x != 0 {
            x /= super::GAME_SIZE as i32;
        }
        if y != 0 {
            y /= super::GAME_SIZE as i32;
        }
        return (x, y);
    }
    
    #[allow(dead_code)]
    pub fn tile_to_world(x: i32, y: i32) -> Vector2f {
        Vector2f::new((x * super::GAME_SIZE as i32) as f32, (y * super::GAME_SIZE as i32) as f32)
    }
}

impl<'a> Drawable for Level<'a> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        let mut vertex_count = 0;
        
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let level_object = &self.map[x as usize][y as usize];
                let texture_rect = self.sprite_sheet.get_background_texture_rect(&level_object.level_type);
                
                // Bottom left
                self.vertex_array.get_vertex(vertex_count + 0).position = Vector2f::new(level_object.position.left,
                                                                                   level_object.position.top + level_object.position.height);
                self.vertex_array.get_vertex(vertex_count + 0).tex_coords = Vector2f::new(texture_rect.left,
                                                                                   texture_rect.top + texture_rect.height);
                
                // Top left
                self.vertex_array.get_vertex(vertex_count + 1).position = Vector2f::new(level_object.position.left,
                                                                                   level_object.position.top);
                self.vertex_array.get_vertex(vertex_count + 1).tex_coords = Vector2f::new(texture_rect.left,
                                                                                     texture_rect.top);
                
                // Top right
                self.vertex_array.get_vertex(vertex_count + 2).position = Vector2f::new(level_object.position.left +
                                                                                   level_object.position.width, level_object.position.top);
                self.vertex_array.get_vertex(vertex_count + 2).tex_coords = Vector2f::new(texture_rect.left +
                                                                                     texture_rect.width, texture_rect.top);
                
                // Bottom right
                self.vertex_array.get_vertex(vertex_count + 3).position = Vector2f::new(level_object.position.left +
                                                                                   level_object.position.width, level_object.position.top + level_object.position.height);
                self.vertex_array.get_vertex(vertex_count + 3).tex_coords = Vector2f::new(texture_rect.left +
                                                                                    texture_rect.width, texture_rect.top + texture_rect.height);
                vertex_count += 4;
            }
        }
        let mut states = RenderStates::default();
        states.texture = Some(&self.sprite_sheet.texture);
        target.draw_with_renderstates(&self.vertex_array, &mut states);
    }
}