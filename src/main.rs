extern crate sfml;

mod player;

use sfml::system::{Vector2f};
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::window::keyboard::Key;
use sfml::graphics::{RenderWindow, RenderTarget, Color};
use player::Player;

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

    while window.is_open() {
    
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::KeyPressed{code, ..} => check_input(&mut window, &mut player, code),
                _ => { /* Nothing */ }
            }
        }
        
        window.clear(&Color::black());
        window.draw(&player);
        window.display();
    }
}

fn check_input(window: &mut RenderWindow, player: &mut Player, key_pressed: Key) {
    
    let move_speed = 5.0;

    match key_pressed {
        Key::Escape => window.close(),
        Key::W => move_player(player, Vector2f::new(0.0, -move_speed)),
        Key::S => move_player(player, Vector2f::new(0.0, move_speed)),
        Key::A => move_player(player, Vector2f::new(-move_speed, 0.0)),
        Key::D => move_player(player, Vector2f::new(move_speed, 0.0)),
        _ => { }
    }
}

fn move_player(player: &mut Player, position: Vector2f) {
    let player_pos = player.shape.get_position() + position;
    player.shape.set_position(&player_pos);
}
