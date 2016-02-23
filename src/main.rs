extern crate sfml;

mod level;
mod player;
mod input;
mod game_time;
mod units;
mod level_object;
mod sprite_sheet;

use sfml::system as sf;
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::window::keyboard::Key;
use sfml::graphics::{Image, RenderWindow, RenderTarget, Color, Text, Font, View};

use player::{Player};
use input::Input;
use game_time::GameTime;
use level::Level;
use sprite_sheet::SpriteSheet;

const TARGET_FPS: i32 = 60;
const MS_PER_UPDATE: units::MS = 1000;
//const MS_PER_FRAME: units::MS = MS_PER_UPDATE / TARGET_FPS;

const SCREEN_SCALE: u32 = 4;
const GAME_SIZE: u32 = 8 * SCREEN_SCALE;

const MAP_PATH: &'static str = "res/game_map.png";


fn main() {
    
    let sprite_sheet = SpriteSheet::new();
    
    let level: Level = Level::new_with_image(&Image::new_from_file(MAP_PATH).unwrap(), &sprite_sheet);
    
    let mut window = match RenderWindow::new(VideoMode::new_init(((level.size.x as u32) * GAME_SIZE),
                                                                 ((level.size.y as u32) * GAME_SIZE), 32),
                                            "Rust-Man",
                                            Close,
                                            &ContextSettings::default()) {

        Some(window) => window,
        None => panic!("Cannot create a new Window.")
    
    };
    window.set_key_repeat_enabled(false);
    

    
    let mut player: Player = Player::new(((level.size.x as f32) * GAME_SIZE as f32) / 2.0,
                                         ((level.size.y as f32) * GAME_SIZE as f32) / 2.0,
                                         &sprite_sheet);
    let mut input: Input = Input::new();
    let mut game_time: GameTime = GameTime::new();
    
    let view: View = View::new_init(&sf::Vector2f::new(window.get_size().x as f32 / 2.0, window.get_size().y as f32 / 2.0),
                                        &sf::Vector2f::new(window.get_size().x as f32, window.get_size().y as f32)).unwrap();

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
        player.update(&input, &game_time, &level);
        //view.set_center(&player.transform.get_position());
        //window.set_view(&view);

        // Fixed Update
        while game_time.fixed_time >= MS_PER_UPDATE as units::DT {
            game_time.fixed_ticks += 1;
            game_time.fixed_time -= MS_PER_UPDATE as units::DT;
            // FixedUpdate()
        }

        // Rendering
        window.set_view(&view);
        window.clear(&Color::black());
        window.draw(&level);
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
            let player_pos = player.transform.get_position();
            fps_text.set_string(&format!("FPS: {} - Player Pos: {}, {}", game_time.fps, player_pos.x, player_pos.y));
        }
    }
}