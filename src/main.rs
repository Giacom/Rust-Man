extern crate sfml;

mod level;
mod player;
mod input;
mod game_time;
mod units;

use sfml::system as sf;
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::window::keyboard::Key;
use sfml::graphics::{RenderWindow, RenderTarget, Color, Text, Font, View};

use player::{Player, MoveDirection};
use input::Input;
use game_time::GameTime;
use level::{Level, LevelObject};

const TARGET_FPS: i32 = 60;
const MS_PER_UPDATE: units::MS = 1000;
const MS_PER_FRAME: units::MS = MS_PER_UPDATE / TARGET_FPS;

const SCREEN_SCALE: u32 = 2;
const GAME_SIZE: u32 = 8 * SCREEN_SCALE;

const WINDOW_WIDTH: u32 = 224 * SCREEN_SCALE;
const WINDOW_HEIGHT: u32 = 288 * SCREEN_SCALE;

const LEVEL_WIDTH: u32 = (WINDOW_WIDTH / GAME_SIZE) - 1;
const LEVEL_HEIGHT: u32 = (WINDOW_HEIGHT / GAME_SIZE) - 1;

fn main() {
    
    let mut window = match RenderWindow::new(VideoMode::new_init(WINDOW_WIDTH, WINDOW_HEIGHT, 32),
                                            "Rust-Man",
                                            Close,
                                            &ContextSettings::default()) {

        Some(window) => window,
        None => panic!("Cannot create a new Window.")
    
    };
    window.set_key_repeat_enabled(false);

    let mut debug_level = Vec::<Vec<LevelObject>>::new();
    
    for x in 0..LEVEL_WIDTH {
        debug_level.push(Vec::<LevelObject>::new());
        for y in 0..LEVEL_HEIGHT {
            if x == y {
                debug_level[x as usize].push(LevelObject::WALL);
            } else {
                debug_level[x as usize].push(LevelObject::SPACE);
            }
        }
    }
    
    let level: Level = Level::new_with_map(debug_level);
    
    let mut player: Player = Player::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0, 16.0, 16.0);
    let mut input: Input = Input::new();
    let mut game_time: GameTime = GameTime::new();
    
    let view: View = View::new_init(&sf::Vector2f::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0),
                                        &sf::Vector2f::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)).unwrap();

    let font: Font = match Font::new_from_file("res/fonts/arial.ttf") {
        Some(font) => font,
        None => panic!("Could not load arial font!")
    };

    let mut fps_text: Text = Text::new_init(&format!("FPS: {}", TARGET_FPS), &font, 10).unwrap(); 
    fps_text.set_color(&Color::yellow());
    
    

    while window.is_open() {

        // Start calculating new time data
        game_time.previous_frame_time = game_time.start_frame_time;
        game_time.start_frame_time = game_time.get_time_in_ms();

        game_time.elapsed_time = game_time.start_frame_time - game_time.previous_frame_time;

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
        
        if input.is_key_down(Key::Escape) {
            window.close();
        }


        // Update
        player.process_input(&input, &game_time);

        // Fixed Update
        while game_time.fixed_time >= MS_PER_UPDATE as units::DT {
            game_time.fixed_ticks += 1;
            game_time.fixed_time -= MS_PER_UPDATE as units::DT;
            // FixedUpdate()
        }

        // Rendering
        window.set_view(&view);
        window.clear(&Color::black());
        window.draw(&player);
        window.draw(&fps_text);
        window.display();

        // VSYNC
/*
        if game_time.elapsed_time < MS_PER_FRAME {
            sf::sleep(sf::Time::with_milliseconds(game_time.start_frame_time + MS_PER_FRAME - game_time.get_time_in_ms()));
        }
*/
        if game_time.elapsed_time != 0 {
            game_time.fps = 1000 / game_time.elapsed_time;
        }

        if game_time.ticks % 100 == 0 {
            let player_pos = player.shape.get_position();
            fps_text.set_string(&format!("FPS: {} - Player Pos: {}, {}", game_time.fps, player_pos.x, player_pos.y));
        }
    }
}