use sfml::system as sf;
use sfml::graphics::{RenderTarget, VertexArray, PrimitiveType, Transformable, RenderStates};
use sfml::traits::Drawable;
use sfml::window::keyboard::Key;

use input::Input;
use game_time::GameTime;
use sprite_sheet::{SpriteSheet, SpriteType};

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
    move_dir: MoveDirection,
    sprite_sheet: &'a SpriteSheet,
    vertex_array: VertexArray
}

impl<'a> Player<'a> {
    pub fn new(x: f32, y: f32, sprite_sheet: &'a SpriteSheet) -> Player<'a> {
        let mut player = Player {
            transform: Transformable::new().unwrap(),
            move_speed: 0.2,
            move_dir: MoveDirection::NONE,
            sprite_sheet: sprite_sheet,
            vertex_array: VertexArray::new_init(PrimitiveType::Quads, 4).unwrap()
        };
        player.transform.set_origin(&sf::Vector2f::new((super::GAME_SIZE / 2) as f32, (super::GAME_SIZE / 2) as f32));
        player.transform.set_position(&sf::Vector2f::new(x, y));
        return player;
    }
    
    pub fn process_input(&mut self, input: &Input, game_time: &GameTime) {
            
        let move_speed = self.move_speed;
        let delta_time = game_time.delta_time;
        
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
        
        let position = match self.move_dir {
            MoveDirection::UP => {
                self.transform.set_rotation(270.0);
                sf::Vector2f::new(0.0, -move_speed * delta_time)
            },
            MoveDirection::DOWN => {
                self.transform.set_rotation(90.0);
                sf::Vector2f::new(0.0, move_speed * delta_time)
            }
            MoveDirection::LEFT => {
                self.transform.set_rotation(180.0);
                sf::Vector2f::new(-move_speed * delta_time, 0.0)
            }
            MoveDirection::RIGHT => {
                self.transform.set_rotation(0.0);
                sf::Vector2f::new(move_speed * delta_time, 0.0)
            }
            _ => sf::Vector2f::new(0.0, 0.0)
        };
        
        self.update_movement(position);
    }
    
    fn update_movement(&mut self, mut movement: sf::Vector2f) {
        movement = self.check_collision(movement);
        let player_pos = self.transform.get_position() + movement;
        self.transform.set_position(&player_pos);
    }
    
    fn check_collision(&self, proposed_movement: sf::Vector2f) -> sf::Vector2f {
        // TODO
        return proposed_movement;
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
