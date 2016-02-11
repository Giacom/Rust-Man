use sfml::system as sf;
use sfml::graphics::{RenderTarget, RectangleShape, Color};
use sfml::traits::Drawable;
use sfml::window::keyboard::Key;

use input::Input;
use game_time::GameTime;

pub enum MoveDirection {
    NONE,
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct Player<'a> {
    pub shape: RectangleShape<'a>,
    pub move_speed: f32,
    pub move_dir: MoveDirection
}

impl<'a> Player<'a> {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Player<'a> {
        let mut player = Player {
            shape: RectangleShape::new().unwrap(),
            move_speed: 0.2,
            move_dir: MoveDirection::NONE
        };

        player.shape.set_size2f(w, h);
        player.shape.set_position2f(x, y);
        player.shape.set_fill_color(&Color::yellow());
        return player;
    }
    
    pub fn process_input(&mut self, input: &Input, game_time: &GameTime) {
            
        let move_speed = self.move_speed;
        let delta_time = game_time.delta_time;
        
        if input.is_key_held_down(Key::W) {
            self.move_dir = MoveDirection::UP;
        }

        if input.is_key_held_down(Key::S) {
            self.move_dir = MoveDirection::DOWN;
        }

        if input.is_key_held_down(Key::A) {
            self.move_dir = MoveDirection::LEFT;
        }

        if input.is_key_held_down(Key::D) {
            self.move_dir = MoveDirection::RIGHT;
        }
        
        let position = match self.move_dir {
            MoveDirection::UP => sf::Vector2f::new(0.0, -move_speed * delta_time),
            MoveDirection::DOWN => sf::Vector2f::new(0.0, move_speed * delta_time),
            MoveDirection::LEFT => sf::Vector2f::new(-move_speed * delta_time, 0.0),
            MoveDirection::RIGHT => sf::Vector2f::new(move_speed * delta_time, 0.0),
            _ => sf::Vector2f::new(0.0, 0.0)
        };
        
        self.update_movement(position);
    }
    
    fn update_movement(&mut self, mut movement: sf::Vector2f) {
        movement = self.check_collision(movement);
        let player_pos = self.shape.get_position() + movement;
        self.shape.set_position(&player_pos);
    }
    
    fn check_collision(&self, proposed_movement: sf::Vector2f) -> sf::Vector2f {
        // TODO
        return proposed_movement;
    }
}

impl<'a> Drawable for Player<'a> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        self.shape.draw(target);
    }
}
