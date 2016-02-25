use sfml::system as sf;
use sfml::graphics::{RenderTarget, VertexArray, PrimitiveType, Sprite, RenderStates, FloatRect};
use sfml::traits::Drawable;
use sfml::window::keyboard::Key;

use input::Input;
use level::Level;
use level_object::{LevelType};
use game_time::GameTime;

#[derive(Clone, PartialEq, Eq)]
pub enum MoveDirection {
    NONE,
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct Player<'s> {
    pub sprite: Sprite<'s>,
    
    move_speed: f32,
    initial_dir: MoveDirection,
    move_dir: MoveDirection,
    vertex_array: VertexArray
}

impl<'s> Player<'s> {
    pub fn new(x: f32, y: f32, sprite: Sprite<'s>) -> Player<'s> {
        let mut player = Player {
            sprite: sprite,
            move_speed: 0.05,
            initial_dir: MoveDirection::NONE,
            move_dir: MoveDirection::NONE,
            vertex_array: VertexArray::new_init(PrimitiveType::Quads, 4).unwrap()
        };
        let local_bounds = player.sprite.get_local_bounds();
        let new_scale = sf::Vector2f::new(super::GAME_SIZE as f32 / local_bounds.width, super::GAME_SIZE as f32 / local_bounds.height);
                
        player.sprite.set_scale(&new_scale);
        
        // Set origin to center
        player.sprite.set_origin2f(local_bounds.width * 0.5, local_bounds.height * 0.5);
        player.sprite.set_position2f(x, y);
        
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
        let player_pos = self.sprite.get_position();

        let mut new_movement = self.check_collision(player_pos, movement, level);
        
        // Try to round our position to 8 when turning to make it easier to enter tunnels
        if self.initial_dir != self.move_dir {
            new_movement = Player::round_movement(new_movement, 8);
        }
        
        self.initial_dir = self.move_dir.clone();
        self.sprite.set_position(&new_movement);
    }
    
    fn get_new_movement(&self, game_time: &GameTime) -> sf::Vector2f {
        match self.move_dir {
            MoveDirection::UP => sf::Vector2f::new(0.0, -self.move_speed * super::SCREEN_SCALE as f32 * game_time.delta_time),
            MoveDirection::DOWN => sf::Vector2f::new(0.0, self.move_speed * super::SCREEN_SCALE as f32 * game_time.delta_time),
            MoveDirection::LEFT => sf::Vector2f::new(-self.move_speed * super::SCREEN_SCALE as f32 * game_time.delta_time, 0.0),
            MoveDirection::RIGHT => sf::Vector2f::new(self.move_speed * super::SCREEN_SCALE as f32 * game_time.delta_time, 0.0),
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
        
        // Player's size, correctly scaled.
        let player_size = self.get_size();

        // Player's origin is different (center instead of bottom left)
        let curr_origin = self.sprite.get_origin() * self.sprite.get_scale();
        
        // Create new global bounds with the new proposed movement.
        let curr_rect = FloatRect::new(proposed_movement.x - curr_origin.x, proposed_movement.y - curr_origin.y,
                                       player_size.x, player_size.y);

        for checking_tile in &checking_tiles {
            
            if checking_tile.level_type == LevelType::WALL {
               
                let proposed_rect = &checking_tile.sprite.get_global_bounds();
                
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
    
    fn get_size(&self) -> sf::Vector2f {
        let player_scale = self.sprite.get_scale();
        let player_bounds = self.sprite.get_local_bounds();
        
        let width = player_bounds.width * player_scale.x;
        let height = player_bounds.height * player_scale.y;
        
        return sf::Vector2f::new(width, height);
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
            MoveDirection::UP => self.sprite.set_rotation(270.0),
            MoveDirection::DOWN => self.sprite.set_rotation(90.0),
            MoveDirection::LEFT => self.sprite.set_rotation(180.0),
            MoveDirection::RIGHT => self.sprite.set_rotation(0.0),
            _ => { }
        }    
    }
}

impl<'s> Drawable for Player<'s> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {

        let sprite_rect = self.sprite.get_local_bounds();
        let texture_rect = self.sprite.get_texture_rect();

        // Bottom left
        self.vertex_array.get_vertex(0).position = sf::Vector2f::new(sprite_rect.left, sprite_rect.top + sprite_rect.height);
        self.vertex_array.get_vertex(0).tex_coords = sf::Vector2f::new(texture_rect.left as f32,
                                                                         (texture_rect.top + texture_rect.height) as f32);
                
        // Top left
        self.vertex_array.get_vertex(1).position = sf::Vector2f::new(sprite_rect.left, sprite_rect.top);
        self.vertex_array.get_vertex(1).tex_coords = sf::Vector2f::new(texture_rect.left as f32,
                                                                       texture_rect.top as f32);


        // Top right
        self.vertex_array.get_vertex(2).position = sf::Vector2f::new(sprite_rect.left + sprite_rect.width, sprite_rect.top);
        self.vertex_array.get_vertex(2).tex_coords = sf::Vector2f::new((texture_rect.left +
                                                                texture_rect.width) as f32, texture_rect.top as f32);
        
        // Bottom right
        self.vertex_array.get_vertex(3).position = sf::Vector2f::new(sprite_rect.left + sprite_rect.width, sprite_rect.top + sprite_rect.height);
        self.vertex_array.get_vertex(3).tex_coords = sf::Vector2f::new((texture_rect.left +
                                                                    texture_rect.width) as f32, (texture_rect.top + texture_rect.height) as f32);  
        let mut states = RenderStates::default();
        states.texture = Some(&self.sprite.get_texture().unwrap());
        states.transform = self.sprite.get_transform();
        target.draw_with_renderstates(&self.vertex_array, &mut states);
    }
}
