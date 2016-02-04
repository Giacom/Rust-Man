extern crate sfml;

mod player;
mod input;
mod game_time;
mod units;


use sfml::system::{Vector2f, sleep, Time};
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::window::keyboard::Key;
use sfml::graphics::{RenderWindow, RenderTarget, Color, Text, Font};

use player::Player;
use input::Input;
use game_time::GameTime;

const TARGET_FPS: i32 = 60;
const MS_PER_UPDATE: units::MS = 1000;
const MS_PER_FRAME: units::MS = MS_PER_UPDATE / TARGET_FPS;

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
    let mut game_time: GameTime = GameTime::new();

    let font: Font = match Font::new_from_file("res/fonts/arial.ttf") {
        Some(font) => font,
        None => panic!("Could not load arial font!")
    };

    let mut fps_text: Text = Text::new_init(&format!("FPS: {}", TARGET_FPS), &font, 10).unwrap(); 
    fps_text.set_color(&Color::yellow());

    while window.is_open() {

        // Start calculating new time data
        game_time.start_frame_time = game_time.get_time_in_ms();

        game_time.elapsed_time = game_time.start_frame_time - game_time.previous_frame_time;

        game_time.previous_frame_time = game_time.start_frame_time;

        game_time.delta_time = game_time.elapsed_time as units::DT;
        game_time.fixed_time += game_time.elapsed_time as units::DT;
        game_time.ticks += 1;

        input.clear_input();

        // Input
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _ => { /* Nothing */ }
            }
            input.check_input(event);
        }

        player_input(&mut window, &mut player, &input, &game_time);

        // Update
        // Update()

        // Fixed Update
        while game_time.fixed_time >= MS_PER_UPDATE as units::DT {
            game_time.fixed_ticks += 1;
            game_time.fixed_time -= MS_PER_UPDATE as units::DT;
            // FixedUpdate()
        }

        // Rendering
        window.clear(&Color::black());
        window.draw(&player);
        window.draw(&fps_text);
        window.display();

        // VSYNC
        game_time.elapsed_time = game_time.get_time_in_ms() - game_time.start_frame_time;

        if game_time.elapsed_time < MS_PER_FRAME {
            sleep(Time::with_milliseconds(MS_PER_FRAME - game_time.elapsed_time));
        }

        game_time.elapsed_time = game_time.get_time_in_ms() - game_time.start_frame_time;
        game_time.fps = 1000 / game_time.elapsed_time;

        if game_time.ticks % 100 == 0 {
            fps_text.set_string(&format!("FPS: {}", game_time.fps));
        }
    }
}

fn player_input(window: &mut RenderWindow, player: &mut Player, input: &Input, game_time: &GameTime) {
    
    let move_speed = 0.5;
    
    if input.is_key_held_down(Key::W) {
        move_player(player, Vector2f::new(0.0, -move_speed * game_time.delta_time));
    }

    if input.is_key_held_down(Key::S) {
        move_player(player, Vector2f::new(0.0, move_speed * game_time.delta_time));
    }

    if input.is_key_held_down(Key::A) {
        move_player(player, Vector2f::new(-move_speed * game_time.delta_time, 0.0));
    }

    if input.is_key_held_down(Key::D) {
        move_player(player, Vector2f::new(move_speed * game_time.delta_time, 0.0));
    }

    if input.is_key_down(Key::Escape) {
        window.close();
    }
}

fn move_player(player: &mut Player, position: Vector2f) {
    let player_pos = player.shape.get_position() + position;
    player.shape.set_position(&player_pos);
}
