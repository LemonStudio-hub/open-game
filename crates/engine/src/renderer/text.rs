use super::sprite::SpriteParams;
use crate::color::Color;
use crate::math::Vec2;

pub struct BitmapFont {
    char_width: f32,
    char_height: f32,
    chars_per_row: u32,
    first_char: char,
}

impl BitmapFont {
    pub fn new(char_width: f32, char_height: f32, chars_per_row: u32, first_char: char) -> Self {
        Self {
            char_width,
            char_height,
            chars_per_row,
            first_char,
        }
    }

    pub fn default_font() -> Self {
        Self {
            char_width: 8.0,
            char_height: 16.0,
            chars_per_row: 16,
            first_char: ' ',
        }
    }

    pub fn char_size(&self) -> Vec2 {
        Vec2::new(self.char_width, self.char_height)
    }

    pub fn char_uv(&self, c: char) -> (Vec2, Vec2) {
        let index = (c as u32).saturating_sub(self.first_char as u32);
        let col = index % self.chars_per_row;
        let row = index / self.chars_per_row;

        let u = col as f32 / self.chars_per_row as f32;
        let v = row as f32 / (self.chars_per_row as f32);
        let u_size = 1.0 / self.chars_per_row as f32;
        let v_size = 1.0 / self.chars_per_row as f32;

        (Vec2::new(u, v), Vec2::new(u_size, v_size))
    }

    pub fn text_width(&self, text: &str) -> f32 {
        text.len() as f32 * self.char_width
    }

    pub fn text_height(&self) -> f32 {
        self.char_height
    }

    pub fn draw_text(
        &self,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        color: Color,
        renderer: &mut super::sprite::SpriteRenderer,
    ) {
        let scale = size / self.char_height;
        let mut cursor_x = x;

        for c in text.chars() {
            if c == '\n' {
                cursor_x = x;
                continue;
            }

            let (uv_min, uv_max) = self.char_uv(c);
            let char_size = Vec2::new(self.char_width * scale, self.char_height * scale);
            let position = Vec2::new(cursor_x + char_size.x * 0.5, y + char_size.y * 0.5);

            renderer.draw_sprite(&SpriteParams {
                position,
                size: char_size,
                color,
                uv_min,
                uv_max,
                ..Default::default()
            });

            cursor_x += char_size.x;
        }
    }
}
