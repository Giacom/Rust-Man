use sfml::system as sf;
use sfml::graphics::{RenderTarget, VertexArray, PrimitiveType, Transformable, RenderStates, FloatRect};
use sfml::traits::Drawable;
use sfml::window::keyboard::Key;

use input::Input;
use level::Level;
use level_object::{LevelType};
use game_time::GameTime;
use sprite_sheet::{SpriteSheet, SpriteType};

#[derive(Clone, PartialEq, Eq)]
pub enum MoveDirection {
    NONE,
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct Player<'a> {
    pub transform: Transformable,
    
    move_speed: f32,
    initial_dir: MoveDirection,
    move_dir: MoveDirection,
    sprite_sheet: &'a SpriteSheet,
    vertex_array: VertexArray
}

impl<'a> Player<'a> {
    pub fn new(x: f32, y: f32, sprite_sheet: &'a SpriteSheet) -> Player<'a> {
        let mut player = Player {
            transform: Transformable::new().unwrap(),
            move_speed: 0.15,
            initial_dir: MoveDirection::NONE,
            move_dir: MoveDirection::NONE,
            sprite_sheet: sprite_sheet,
            vertex_array: VertexArray::new_init(PrimitiveType::Quads, 4).unwrap()
        };
        player.transform.set_origin(&sf::Vector2f::new((super::GAME_SIZE / 2) as f32, (super::GAME_SIZE / 2) as f32));
        player.transform.set_position(&sf::Vector2f::new(x, y));
        return player;
    }
    
    pub fn update(&mut self, input: &Input, game_time: &GameTime, level: &Level) {
        self.process_input(input);
        self.update_movement(game_time, level);
        self.update_rotation();
    }
    
    pub fn process_input(&mut self, input: &Input) {

        self.initial_dir = self.move_dir.clone();
        
        if input.is_key_down(Key::W) {
            self.move_dir = MoveDirection::UP;
        }

        if input.is_key_down(Key::S) {
            self.move_dir = MoveDirection::DOWN;
        }

        if input.is_key_down(Key::A) {
            self.move_dir = MoveDirection::LEFT;
        }

        if input.is_key_down(Key::D) {
            self.move_dir = MoveDirection::RIGHT;
        }
    }
    
    fn update_movement(&mut self, game_time: &GameTime, level: &Level) {
    
        let movement = self.get_new_movement(game_time);
        let player_pos = self.transform.get_position();
        
        let mut new_movement = self.check_collision(player_pos, movement, level);
        
        // Try to round our position to 8 when turning to make it easier to enter tunnels
        if self.initial_dir != self.move_dir {
            new_movement = Player::round_movement(new_movement, 8);
        }
        
        self.initial_dir = self.move_dir.clone();
        self.transform.set_position(&new_movement);
    }
    
    fn get_new_movement(&self, game_time: &GameTime) -> sf::Vector2f {
        match self.move_dir {
            MoveDirection::UP => sf::Vector2f::new(0.0, -self.move_speed * game_time.delta_time),
            MoveDirection::DOWN => sf::Vector2f::new(0.0, self.move_speed * game_time.delta_time),
            MoveDirection::LEFT => sf::Vector2f::new(-self.move_speed * game_time.delta_time, 0.0),
            MoveDirection::RIGHT => sf::Vector2f::new(self.move_speed * game_time.delta_time, 0.0),
            _ => sf::Vector2f::new(0.0, 0.0)
        }
    }
    
    fn check_collision(&mut self, player_pos: sf::Vector2f, new_movement: sf::Vector2f, level: &Level) -> sf::Vector2f {

        let (x, y) = Level::world_to_tile(player_pos.x, player_pos.y);
        let mut proposed_movement = player_pos + new_movement;
        
        // Check all tiles around the player
        let checking_tiles = vec![level.get_tile(x, y - 1), // Up
                                  level.get_tile(x, y + 1), // Down
                                  level.get_tile(x - 1, y), // Left
                                  level.get_tile(x + 1, y), // Right
                                  level.get_tile(x - 1, y - 1), // Up-Left
                                  level.get_tile(x + 1, y - 1), // Up-Right
                                  level.get_tile(x - 1, y + 1), // Down-Left
                                  level.get_tile(x + 1, y + 1)]; // Down-Right
        

        // Player's origin is different (center instead of bottom left)
        let curr_origin = self.transform.get_origin();
        let curr_rect = FloatRect::new(proposed_movement.x - curr_origin.x, proposed_movement.y - curr_origin.y,
                                       super::GAME_SIZE as f32, super::GAME_SIZE as f32);

        for checking_tile in &checking_tiles {
            
            if checking_tile.level_type == LevelType::WALL {
               
                let proposed_rect = &checking_tile.position;
                
                // Not used since RSFML doesn't have the intersect function which ommits requiring it
                let mut overlapped_rect = FloatRect::new(0.0, 0.0, 0.0, 0.0);
                
                if FloatRect::intersects(&curr_rect, &proposed_rect, &mut overlapped_rect) {
                    match self.move_dir {
                        MoveDirection::UP  => proposed_movement.y = proposed_rect.top + proposed_rect.height + curr_origin.y,
                        MoveDirection::DOWN => proposed_movement.y = proposed_rect.top - curr_origin.y,
                        MoveDirection::LEFT => proposed_movement.x = proposed_rect.left + proposed_rect.width + curr_origin.x,
                        MoveDirection::RIGHT => proposed_movement.x = proposed_rect.left - curr_origin.x,
                        MoveDirection::NONE => { }   
                    }
                    break;
                }
            }
        }
        
        return proposed_movement;
    }
    
    fn round_movement(movement: sf::Vector2f, num_round_to: i32) -> sf::Vector2f {
        // Rounds to a multiple of a number
        let round_num_to = |num: f32, round_to: i32| {
            let num = num.round() as i32;
            let remainder = num % round_to;
            if remainder >= (round_to / 2) { (num - remainder + round_to) as f32 } else { (num - remainder) as f32 }
        };
        return sf::Vector2f::new(round_num_to(movement.x, num_round_to), round_num_to(movement.y, num_round_to));
    }
   
    fn update_rotation(&mut self) {
        match self.move_dir {
            MoveDirection::UP => self.transform.set_rotation(270.0),
            MoveDirection::DOWN => self.transform.set_rotation(90.0),
            MoveDirection::LEFT => self.transform.set_rotation(180.0),
            MoveDirection::RIGHT => self.transform.set_rotation(0.0),
            _ => { }
        }    
    }
}

impl<'a> Drawable for Player<'a> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {

        let texture_rect = self.sprite_sheet.get_foreground_texture_rect(&SpriteType::PLAYER);

        // Bottom left
        self.vertex_array.get_vertex(0).position = sf::Vector2f::new(0.0, super::GAME_SIZE as f32);
        self.vertex_array.get_vertex(0).tex_coords = sf::Vector2f::new(texture_rect.left,
                                                                         texture_rect.top + texture_rect.height);
                
        // Top left
        self.vertex_array.get_vertex(1).position = sf::Vector2f::new(0.0, 0.0);
        self.vertex_array.get_vertex(1).tex_coords = sf::Vector2f::new(texture_rect.left,
                                                                       texture_rect.top);


        // Top right
        self.vertex_array.get_vertex(2).position = sf::Vector2f::new(super::GAME_SIZE as f32, 0.0);
        self.vertex_array.get_vertex(2).tex_coords = sf::Vector2f::new(texture_rect.left +
                                                                texture_rect.width, texture_rect.top);
        
        // Bottom right
        self.vertex_array.get_vertex(3).position = sf::Vector2f::new(super::GAME_SIZE as f32, super::GAME_SIZE as f32);
        self.vertex_array.get_vertex(3).tex_coords = sf::Vector2f::new(texture_rect.left +
                                                                    texture_rect.width, texture_rect.top + texture_rect.height);  
        let mut states = RenderStates::default();
        states.texture = Some(&self.sprite_sheet.texture);
        states.transform = self.transform.get_transform();
        target.draw_with_renderstates(&self.vertex_array, &mut states);
    }
}
