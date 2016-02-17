use sfml::system::{Vector2f};
use sfml::graphics::{RenderStates, RenderTarget, FloatRect, VertexArray, PrimitiveType};
use sfml::traits::Drawable;

use level_object::{LevelObject, LevelType};
use sprite_sheet::SpriteSheet;

pub struct Level {
    pub height: usize,
    pub width: usize,
    map: Vec<Vec<LevelObject>>,
    sprite_sheet: SpriteSheet,
    vertex_array: VertexArray
}

impl Level {
 
    pub fn new_with_map(height: usize, width: usize, map: Vec<Vec<LevelType>>) -> Level {
        let new_map = Level::setup_map(&map);

        Level {
            width: width,
            height: height,
            map: new_map,
            sprite_sheet: SpriteSheet::new(),
            vertex_array: VertexArray::new_init(PrimitiveType::Quads, (width * height * 4) as u32).unwrap()
        }
    }
    
    pub fn new_with_text(text: &str) -> Level {
        println!("Loading map:\n\n{}\n", text);

        let mut map = vec![];
        
        let lines: Vec<&str> = text.lines().collect();
        
        let level_height = lines.len();
        let level_width = lines.first().unwrap().len();
        
        println!("Level Width: {} - Level Height: {}", level_width, level_height);
        
        let mut text_map = Vec::<Vec<char>>::new();
        
        for line in &lines {
            let line_map: Vec<char> = line.chars().collect();
            text_map.push(line_map);
        }

        for x in 0..level_width {
            map.push(vec![]);
            for y in 0..level_height {
                let level_type = match text_map[y][x] {
                    '#' => LevelType::WALL,
                    _ => LevelType::SPACE
                };
                map.last_mut().unwrap().push(level_type);
            }
        }

        Level::new_with_map(level_height, level_width, map)
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
        let mut vertex_count = 0;
        
        for x in 0..self.width {
            for y in 0..self.height {
                let level_object = &self.map[x][y];
                let texture_rect = self.sprite_sheet.get_texture_rect(&level_object.level_type);
                
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