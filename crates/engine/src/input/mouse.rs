use std::collections::HashSet;
use crate::math::Vec2;
use super::keys::MouseButton;

pub struct MouseState {
    pub(crate) position: Vec2,
    delta: Vec2,
    pub(crate) buttons_down: HashSet<MouseButton>,
    pub(crate) buttons_pressed: HashSet<MouseButton>,
    pub(crate) buttons_released: HashSet<MouseButton>,
    buttons_down_prev: HashSet<MouseButton>,
    wheel_delta: f32,
    is_locked: bool,
}

impl MouseState {
    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
            delta: Vec2::ZERO,
            buttons_down: HashSet::new(),
            buttons_pressed: HashSet::new(),
            buttons_released: HashSet::new(),
            buttons_down_prev: HashSet::new(),
            wheel_delta: 0.0,
            is_locked: false,
        }
    }

    pub fn on_move(&mut self, x: f32, y: f32) {
        self.position = Vec2::new(x, y);
    }

    pub fn on_move_delta(&mut self, dx: f32, dy: f32) {
        self.delta += Vec2::new(dx, dy);
    }

    pub fn on_button_down(&mut self, button: MouseButton) {
        self.buttons_down.insert(button);
    }

    pub fn on_button_up(&mut self, button: MouseButton) {
        self.buttons_down.remove(&button);
    }

    pub fn on_wheel(&mut self, delta: f32) {
        self.wheel_delta += delta;
    }

    pub fn on_locked(&mut self, locked: bool) {
        self.is_locked = locked;
    }

    pub fn update(&mut self) {
        self.buttons_pressed.clear();
        self.buttons_released.clear();

        for button in &self.buttons_down {
            if !self.buttons_down_prev.contains(button) {
                self.buttons_pressed.insert(*button);
            }
        }

        for button in &self.buttons_down_prev {
            if !self.buttons_down.contains(button) {
                self.buttons_released.insert(*button);
            }
        }

        self.buttons_down_prev = self.buttons_down.clone();
        self.delta = Vec2::ZERO;
        self.wheel_delta = 0.0;
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn delta(&self) -> Vec2 {
        self.delta
    }

    pub fn wheel_delta(&self) -> f32 {
        self.wheel_delta
    }

    pub fn is_button_down(&self, button: MouseButton) -> bool {
        self.buttons_down.contains(&button)
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.buttons_pressed.contains(&button)
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        self.buttons_released.contains(&button)
    }

    pub fn is_locked(&self) -> bool {
        self.is_locked
    }

    pub fn clear(&mut self) {
        self.buttons_down.clear();
        self.buttons_pressed.clear();
        self.buttons_released.clear();
        self.buttons_down_prev.clear();
        self.delta = Vec2::ZERO;
        self.wheel_delta = 0.0;
    }
}

impl Default for MouseState {
    fn default() -> Self {
        Self::new()
    }
}
