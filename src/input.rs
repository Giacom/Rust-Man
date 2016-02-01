use sfml::window::event;
use sfml::window::event::Event;
use sfml::window::keyboard::Key;
use std::collections::HashMap;

// Day 2 - Currently stuck on working out making a non-hashable enum (Key),
// from a library, hashable.
//
// Tried wrapping HKey in a struct but it results in:
// error: the trait `core::hash::Hash` is not implemented for the type
// `sfml::window::keyboard::Key`
#[derive(Hash, Eq, PartialEq)]
struct HKey {
     key: Key
}

pub struct Input {
    held_keys: HashMap<HKey, bool>,
    pressed_keys: HashMap<HKey, bool>,
    released_keys: HashMap<HKey, bool>
}

impl Input {

    pub fn check_input(&self, event: &Event) {
        match event {
            &event::KeyPressed{code, ..} => self.key_pressed(code),
            &event::KeyReleased{code, ..} => self.key_released(code),
            _ => { }
        }
    }

    pub fn key_pressed(&self, key: Key) {
        self.pressed_keys.insert(HKey { key: key }, true);
        self.held_keys.insert(HKey { key: key }, true);
    }

    pub fn key_released(&self, key: Key) {
        // TODO: Once the hashing has been sorted out.
    }
}
