use sfml::window::event;
use sfml::window::event::Event;
use sfml::window::keyboard::Key;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq)]
struct HKey(Key);

impl Hash for HKey {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
            (self.0 as u32).hash(state)
    }
}

pub struct Input {
    held_keys: HashMap<HKey, bool>,
    keys_down: HashMap<HKey, bool>,
    keys_up: HashMap<HKey, bool>
}

impl Input {

    pub fn new() -> Input {
        Input {
            held_keys: HashMap::new(),
            keys_down: HashMap::new(),
            keys_up: HashMap::new(),
        }
    }

    pub fn clear_input(&mut self) {
        self.keys_down.clear();
        self.keys_up.clear();
    }

    pub fn check_input(&mut self, event: Event) {
        match event {
            event::KeyPressed{code, ..} => self.key_pressed(code),
            event::KeyReleased{code, ..} => self.key_released(code),
            _ => { }
        }
    }

    pub fn is_key_held_down(&self, key: Key) -> bool {
        (*self.held_keys.get(&HKey(key)).unwrap_or(&false) == true)
    }

    pub fn is_key_down(&self, key: Key) -> bool { 
        (*self.keys_down.get(&HKey(key)).unwrap_or(&false) == true)
    }

    #[allow(dead_code)]
    pub fn is_key_up(&self, key: Key) -> bool {
        (*self.keys_up.get(&HKey(key)).unwrap_or(&false) == true)
    }

    fn key_pressed(&mut self, key: Key) {
        self.keys_down.insert(HKey(key), true);
        self.held_keys.insert(HKey(key), true);
    }

    fn key_released(&mut self, key: Key) {
        self.keys_up.insert(HKey(key), true);
        self.held_keys.insert(HKey(key), false);
    }
}
