use crate::math::Vec2;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TouchPoint {
    pub id: i32,
    pub position: Vec2,
    pub force: f32,
}

pub struct TouchState {
    pub(crate) touches: HashMap<i32, TouchPoint>,
    touches_began: HashMap<i32, TouchPoint>,
    touches_ended: HashMap<i32, TouchPoint>,
}

impl TouchState {
    pub fn new() -> Self {
        Self {
            touches: HashMap::new(),
            touches_began: HashMap::new(),
            touches_ended: HashMap::new(),
        }
    }

    pub fn on_touch_start(&mut self, id: i32, x: f32, y: f32, force: f32) {
        let point = TouchPoint {
            id,
            position: Vec2::new(x, y),
            force,
        };
        self.touches.insert(id, point.clone());
        self.touches_began.insert(id, point);
    }

    pub fn on_touch_move(&mut self, id: i32, x: f32, y: f32, force: f32) {
        if let Some(touch) = self.touches.get_mut(&id) {
            touch.position = Vec2::new(x, y);
            touch.force = force;
        }
    }

    pub fn on_touch_end(&mut self, id: i32) {
        if let Some(touch) = self.touches.remove(&id) {
            self.touches_ended.insert(id, touch);
        }
    }

    pub fn update(&mut self) {
        self.touches_began.clear();
        self.touches_ended.clear();
    }

    pub fn touches(&self) -> &HashMap<i32, TouchPoint> {
        &self.touches
    }

    pub fn active_touch_count(&self) -> usize {
        self.touches.len()
    }

    pub fn get_touch(&self, id: i32) -> Option<&TouchPoint> {
        self.touches.get(&id)
    }

    pub fn touches_began(&self) -> &HashMap<i32, TouchPoint> {
        &self.touches_began
    }

    pub fn touches_ended(&self) -> &HashMap<i32, TouchPoint> {
        &self.touches_ended
    }

    pub fn clear(&mut self) {
        self.touches.clear();
        self.touches_began.clear();
        self.touches_ended.clear();
    }
}

impl Default for TouchState {
    fn default() -> Self {
        Self::new()
    }
}
