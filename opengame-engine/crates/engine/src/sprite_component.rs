use crate::color::Color;
use crate::math::Vec2;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub texture_id: Option<u32>,
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub visible: bool,
    pub layer: i32,
    pub size: Option<Vec2>,
    pub anchor: Vec2,
}

impl Sprite {
    pub fn new() -> Self {
        Self {
            texture_id: None,
            color: Color::WHITE,
            flip_x: false,
            flip_y: false,
            visible: true,
            layer: 0,
            size: None,
            anchor: Vec2::new(0.5, 0.5),
        }
    }

    pub fn with_texture(mut self, texture_id: u32) -> Self {
        self.texture_id = Some(texture_id);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_layer(mut self, layer: i32) -> Self {
        self.layer = layer;
        self
    }

    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.anchor = anchor;
        self
    }
}

impl Default for Sprite {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub texture_id: u32,
    pub frame_width: f32,
    pub frame_height: f32,
    pub columns: u32,
    pub rows: u32,
    pub padding: f32,
    pub offset: Vec2,
}

impl SpriteSheet {
    pub fn new(
        texture_id: u32,
        frame_width: f32,
        frame_height: f32,
        columns: u32,
        rows: u32,
    ) -> Self {
        Self {
            texture_id,
            frame_width,
            frame_height,
            columns,
            rows,
            padding: 0.0,
            offset: Vec2::ZERO,
        }
    }

    pub fn frame_uv(&self, index: u32) -> (Vec2, Vec2) {
        let col = index % self.columns;
        let row = index / self.columns;
        let u = (col as f32 * (self.frame_width + self.padding) + self.offset.x)
            / (self.columns as f32 * (self.frame_width + self.padding));
        let v = (row as f32 * (self.frame_height + self.padding) + self.offset.y)
            / (self.rows as f32 * (self.frame_height + self.padding));
        let u_size = self.frame_width / (self.columns as f32 * (self.frame_width + self.padding));
        let v_size = self.frame_height / (self.rows as f32 * (self.frame_height + self.padding));
        (Vec2::new(u, v), Vec2::new(u_size, v_size))
    }

    pub fn total_frames(&self) -> u32 {
        self.columns * self.rows
    }
}
