extern crate sfml;

mod player;
mod input;

use sfml::system::{Vector2f};
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::window::keyboard::Key;
use sfml::graphics::{RenderWindow, RenderTarget, Color};
use player::Player;
use input::Input;

fn main() {

    let window_width = 800.0;
    let window_height = 600.0;

    let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                            "Rust-Man",
                                            Close,
                                            &ContextSettings::default()) {

        Some(window) => window,
        None => panic!("Cannot create a new Window.")
    
    };

    let mut player: Player = Player::new(window_width / 2.0, window_height / 2.0, 32.0, 32.0);
    let mut input: Input = Input::new();

    while window.is_open() {
        input.clear_input(); 
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _ => { /* Nothing */ }
            }
            input.check_input(event);
        }

        player_input(&mut window, &mut player, &input);

        window.clear(&Color::black());
        window.draw(&player);
        window.display();
    }
}

fn player_input(window: &mut RenderWindow, player: &mut Player, input: &Input) {
    
    let move_speed = 0.05;
    
    if input.is_key_held_down(Key::W) {
        move_player(player, Vector2f::new(0.0, -move_speed));
    }

    if input.is_key_held_down(Key::S) {
        move_player(player, Vector2f::new(0.0, move_speed));
    }

    if input.is_key_held_down(Key::A) {
        move_player(player, Vector2f::new(-move_speed, 0.0));
    }

    if input.is_key_held_down(Key::D) {
        move_player(player, Vector2f::new(move_speed, 0.0));
    }

    if input.is_key_down(Key::Escape) {
        window.close();
    }
}

fn move_player(player: &mut Player, position: Vector2f) {
    let player_pos = player.shape.get_position() + position;
    player.shape.set_position(&player_pos);
}
