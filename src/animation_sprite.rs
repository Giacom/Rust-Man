

use sfml::graphics::{Sprite, IntRect, FloatRect};

use game_time::GameTime;
use units;

pub struct AnimationSprite<'s> {
    pub sfml_sprite: Sprite<'s>,
    animation_frames: Vec<IntRect>,
    
    current_animation_frame: usize,
    frame_delay: units::MS,
    last_time_animation_changed: units::MS,
    looping: bool,
}

impl<'s> AnimationSprite<'s> {
    pub fn new(sprite: Sprite<'s>, frames: Vec<IntRect>) -> AnimationSprite<'s> {
        AnimationSprite {
            sfml_sprite: sprite,
            animation_frames: frames,
            
            current_animation_frame: 0,
            frame_delay: 100,
            last_time_animation_changed: 0,
            looping: true
        }
    }
    
    pub fn start_animation(&mut self) {
        self.looping = true
    }
    
    pub fn stop_animation(&mut self) {
        self.looping = false;
    }
    
    pub fn reset_animation(&mut self) {
        self.current_animation_frame = 0;
        self.last_time_animation_changed = 0;
    }
    
    pub fn update(&mut self, game_time: &GameTime) {
        if game_time.start_frame_time > self.last_time_animation_changed + self.frame_delay {
            if self.looping {
                self.current_animation_frame = (self.current_animation_frame + 1) % (self.animation_frames.len() * 4);
                
                let animation_index = AnimationSprite::ping_pong(self.current_animation_frame as i32, (self.animation_frames.len() - 1) as i32);       
                self.sfml_sprite.set_texture_rect(&self.animation_frames[animation_index as usize]);
            }
            self.last_time_animation_changed = game_time.start_frame_time;
        }
    }
    
    fn ping_pong(value: i32, length: i32) -> i32 {
        let double_length = 2 * length;
        let wrapped_value = value % double_length;
        
        if 0 <= wrapped_value && wrapped_value < length && 0 <= length {
            return wrapped_value
        } else {
            return double_length - wrapped_value;
        }
    }
}