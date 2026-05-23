use super::keys::KeyCode;
use std::collections::HashSet;

pub struct KeyboardState {
    pub(crate) keys_down: HashSet<KeyCode>,
    pub(crate) keys_pressed: HashSet<KeyCode>,
    pub(crate) keys_released: HashSet<KeyCode>,
    pub(crate) keys_down_prev: HashSet<KeyCode>,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            keys_down_prev: HashSet::new(),
        }
    }

    pub fn on_key_down(&mut self, key: KeyCode) {
        self.keys_down.insert(key);
    }

    pub fn on_key_up(&mut self, key: KeyCode) {
        self.keys_down.remove(&key);
    }

    pub fn update(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();

        for key in &self.keys_down {
            if !self.keys_down_prev.contains(key) {
                self.keys_pressed.insert(*key);
            }
        }

        for key in &self.keys_down_prev {
            if !self.keys_down.contains(key) {
                self.keys_released.insert(*key);
            }
        }

        self.keys_down_prev = self.keys_down.clone();
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: KeyCode) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn any_key_down(&self) -> bool {
        !self.keys_down.is_empty()
    }

    pub fn clear(&mut self) {
        self.keys_down.clear();
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.keys_down_prev.clear();
    }
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self::new()
    }
}
