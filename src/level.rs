use std::cmp;

use sfml::system::{Vector2f, Vector2u};
use sfml::graphics::{Color, RenderStates, RenderTarget, VertexArray, PrimitiveType, Image, Texture};
use sfml::traits::Drawable;

use level_object::{LevelObject, LevelType};
use sprite_sheet::SpriteSheet;

pub struct Level<'s> {
    pub size: Vector2u,
    map: Vec<Vec<LevelObject<'s>>>,
    vertex_array: VertexArray,
    texture: &'s Texture
}

impl<'s> Level<'s> {
 
    pub fn new_with_map(size: Vector2u, map: Vec<Vec<LevelType>>, sprite_sheet: &'s SpriteSheet) -> Level<'s> {
        let new_map = Level::setup_map(&map, sprite_sheet);

        Level {
            size: size,
            map: new_map,
            vertex_array: VertexArray::new_init(PrimitiveType::Quads, (size.x * size.y * 4)).unwrap(),
            texture: &sprite_sheet.texture
        }
    }
    
    pub fn new_with_image(image: &Image, sprite_sheet: &'s SpriteSheet) -> Level<'s> {
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
    
    pub fn setup_map(map: &Vec<Vec<LevelType>>, sprite_sheet: &'s SpriteSheet) -> Vec<Vec<LevelObject<'s>>> {

        let mut returned_map = vec![vec![]];
        
        for x in 0..map.len() {
            returned_map.push(Vec::<LevelObject<'s>>::new());
            for y in 0..map[0].len() {
                let level_type = map[x][y].clone();
                let mut sprite = sprite_sheet.generate_background_sprite(&level_type);
                
                let local_bounds = sprite.get_local_bounds();
                let new_scale = Vector2f::new(super::GAME_SIZE as f32 / local_bounds.width, super::GAME_SIZE as f32 / local_bounds.height);
        
                sprite.set_scale(&new_scale);
                sprite.set_position(&Level::tile_to_world(x as i32, y as i32));
                
                let level_object = LevelObject::new(level_type, sprite);
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
    
    pub fn tile_to_world(x: i32, y: i32) -> Vector2f {
        Vector2f::new((x * super::GAME_SIZE as i32) as f32, (y * super::GAME_SIZE as i32) as f32)
    }
}

impl<'s> Drawable for Level<'s> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        let mut vertex_count = 0;
        
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let level_object = &self.map[x as usize][y as usize];
                
                let level_object_bounds = level_object.sprite.get_global_bounds();
                let texture_rect = level_object.sprite.get_texture_rect();
                
                // Bottom left
                self.vertex_array.get_vertex(vertex_count + 0).position = Vector2f::new(level_object_bounds.left,
                                                                                   level_object_bounds.top + level_object_bounds.height);
                self.vertex_array.get_vertex(vertex_count + 0).tex_coords = Vector2f::new(texture_rect.left as f32,
                                                                                   (texture_rect.top + texture_rect.height) as f32);
                
                // Top left
                self.vertex_array.get_vertex(vertex_count + 1).position = Vector2f::new(level_object_bounds.left,
                                                                                   level_object_bounds.top);
                self.vertex_array.get_vertex(vertex_count + 1).tex_coords = Vector2f::new(texture_rect.left as f32,
                                                                                     texture_rect.top as f32);
                
                // Top right
                self.vertex_array.get_vertex(vertex_count + 2).position = Vector2f::new(level_object_bounds.left +
                                                                                   level_object_bounds.width, level_object_bounds.top);
                self.vertex_array.get_vertex(vertex_count + 2).tex_coords = Vector2f::new((texture_rect.left +
                                                                                     texture_rect.width) as f32, texture_rect.top as f32);
                
                // Bottom right
                self.vertex_array.get_vertex(vertex_count + 3).position = Vector2f::new(level_object_bounds.left +
                                                                                   level_object_bounds.width, level_object_bounds.top + level_object_bounds.height);
                self.vertex_array.get_vertex(vertex_count + 3).tex_coords = Vector2f::new((texture_rect.left +
                                                                                    texture_rect.width) as f32, (texture_rect.top + texture_rect.height) as f32);
                vertex_count += 4;
            }
        }
        let mut states = RenderStates::default();
        states.texture = Some(self.texture);
        target.draw_with_renderstates(&self.vertex_array, &mut states);
    }
}