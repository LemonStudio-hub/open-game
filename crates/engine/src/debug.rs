use crate::color::Color;
use crate::math::Vec2;

#[derive(Debug, Clone)]
pub struct DebugOverlay {
    pub visible: bool,
    pub show_fps: bool,
    pub show_entity_count: bool,
    pub show_frame_time: bool,
    pub show_profiler: bool,
    pub text_color: Color,
    pub bg_color: Color,
    pub padding: f32,
    pub font_size: f32,
    pub position: DebugPosition,
    fps: f32,
    frame_time_ms: f32,
    entity_count: usize,
    component_type_count: usize,
    profiler_lines: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebugPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl DebugOverlay {
    pub fn new() -> Self {
        Self {
            visible: true,
            show_fps: true,
            show_entity_count: true,
            show_frame_time: true,
            show_profiler: false,
            text_color: Color::GREEN,
            bg_color: Color::new(0.0, 0.0, 0.0, 0.7),
            padding: 8.0,
            font_size: 16.0,
            position: DebugPosition::TopLeft,
            fps: 0.0,
            frame_time_ms: 0.0,
            entity_count: 0,
            component_type_count: 0,
            profiler_lines: Vec::new(),
        }
    }

    pub fn update_stats(
        &mut self,
        fps: f32,
        frame_time_ms: f32,
        entity_count: usize,
        component_type_count: usize,
    ) {
        self.fps = fps;
        self.frame_time_ms = frame_time_ms;
        self.entity_count = entity_count;
        self.component_type_count = component_type_count;
    }

    pub fn set_profiler_lines(&mut self, lines: Vec<String>) {
        self.profiler_lines = lines;
    }

    pub fn text_lines(&self) -> Vec<String> {
        if !self.visible {
            return Vec::new();
        }

        let mut lines = Vec::new();

        if self.show_fps {
            lines.push(format!("FPS: {:.1}", self.fps));
        }

        if self.show_frame_time {
            lines.push(format!("Frame: {:.2}ms", self.frame_time_ms));
        }

        if self.show_entity_count {
            lines.push(format!(
                "Entities: {} | Components: {}",
                self.entity_count, self.component_type_count
            ));
        }

        if self.show_profiler {
            for line in &self.profiler_lines {
                lines.push(line.clone());
            }
        }

        lines
    }

    pub fn bg_size(&self, char_width: f32, char_height: f32) -> Vec2 {
        let lines = self.text_lines();
        if lines.is_empty() {
            return Vec2::ZERO;
        }

        let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let width = max_len as f32 * char_width + self.padding * 2.0;
        let height = lines.len() as f32 * char_height + self.padding * 2.0;
        Vec2::new(width, height)
    }

    pub fn bg_position(&self, screen_width: f32, screen_height: f32, bg_size: Vec2) -> Vec2 {
        match self.position {
            DebugPosition::TopLeft => Vec2::new(0.0, 0.0),
            DebugPosition::TopRight => Vec2::new(screen_width - bg_size.x, 0.0),
            DebugPosition::BottomLeft => Vec2::new(0.0, screen_height - bg_size.y),
            DebugPosition::BottomRight => {
                Vec2::new(screen_width - bg_size.x, screen_height - bg_size.y)
            }
        }
    }

    pub fn text_start_position(
        &self,
        screen_width: f32,
        screen_height: f32,
        bg_size: Vec2,
    ) -> Vec2 {
        let bg_pos = self.bg_position(screen_width, screen_height, bg_size);
        Vec2::new(bg_pos.x + self.padding, bg_pos.y + self.padding)
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn toggle_fps(&mut self) {
        self.show_fps = !self.show_fps;
    }

    pub fn toggle_profiler(&mut self) {
        self.show_profiler = !self.show_profiler;
    }
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_overlay_new() {
        let overlay = DebugOverlay::new();
        assert!(overlay.visible);
        assert!(overlay.show_fps);
        assert!(overlay.show_entity_count);
        assert!(overlay.show_frame_time);
        assert!(!overlay.show_profiler);
    }

    #[test]
    fn test_debug_overlay_default() {
        let overlay = DebugOverlay::default();
        assert!(overlay.visible);
    }

    #[test]
    fn test_update_stats() {
        let mut overlay = DebugOverlay::new();
        overlay.update_stats(60.0, 16.67, 100, 5);
        assert!((overlay.fps - 60.0).abs() < f32::EPSILON);
        assert!((overlay.frame_time_ms - 16.67).abs() < 0.01);
        assert_eq!(overlay.entity_count, 100);
    }

    #[test]
    fn test_text_lines_when_visible() {
        let mut overlay = DebugOverlay::new();
        overlay.update_stats(60.0, 16.67, 50, 3);
        let lines = overlay.text_lines();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].contains("FPS: 60.0"));
        assert!(lines[1].contains("16.67"));
        assert!(lines[2].contains("Entities: 50"));
    }

    #[test]
    fn test_text_lines_when_hidden() {
        let mut overlay = DebugOverlay::new();
        overlay.visible = false;
        overlay.update_stats(60.0, 16.67, 50, 3);
        let lines = overlay.text_lines();
        assert!(lines.is_empty());
    }

    #[test]
    fn test_toggle() {
        let mut overlay = DebugOverlay::new();
        assert!(overlay.visible);
        overlay.toggle();
        assert!(!overlay.visible);
        overlay.toggle();
        assert!(overlay.visible);
    }

    #[test]
    fn test_toggle_fps() {
        let mut overlay = DebugOverlay::new();
        assert!(overlay.show_fps);
        overlay.toggle_fps();
        assert!(!overlay.show_fps);
    }

    #[test]
    fn test_toggle_profiler() {
        let mut overlay = DebugOverlay::new();
        assert!(!overlay.show_profiler);
        overlay.toggle_profiler();
        assert!(overlay.show_profiler);
    }

    #[test]
    fn test_profiler_lines() {
        let mut overlay = DebugOverlay::new();
        overlay.show_profiler = true;
        overlay.set_profiler_lines(vec![
            "  render: 8.0ms".to_string(),
            "  update: 2.0ms".to_string(),
        ]);
        overlay.update_stats(60.0, 16.67, 50, 3);
        let lines = overlay.text_lines();
        assert_eq!(lines.len(), 5);
        assert!(lines[3].contains("render"));
        assert!(lines[4].contains("update"));
    }

    #[test]
    fn test_bg_size() {
        let mut overlay = DebugOverlay::new();
        overlay.update_stats(60.0, 16.67, 50, 3);
        let size = overlay.bg_size(8.0, 16.0);
        assert!(size.x > 0.0);
        assert!(size.y > 0.0);
    }

    #[test]
    fn test_bg_position_top_left() {
        let overlay = DebugOverlay::new();
        let pos = overlay.bg_position(800.0, 600.0, Vec2::new(200.0, 100.0));
        assert_eq!(pos, Vec2::ZERO);
    }

    #[test]
    fn test_bg_position_top_right() {
        let mut overlay = DebugOverlay::new();
        overlay.position = DebugPosition::TopRight;
        let pos = overlay.bg_position(800.0, 600.0, Vec2::new(200.0, 100.0));
        assert!((pos.x - 600.0).abs() < f32::EPSILON);
        assert!((pos.y - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_bg_position_bottom_left() {
        let mut overlay = DebugOverlay::new();
        overlay.position = DebugPosition::BottomLeft;
        let pos = overlay.bg_position(800.0, 600.0, Vec2::new(200.0, 100.0));
        assert!((pos.x - 0.0).abs() < f32::EPSILON);
        assert!((pos.y - 500.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_bg_position_bottom_right() {
        let mut overlay = DebugOverlay::new();
        overlay.position = DebugPosition::BottomRight;
        let pos = overlay.bg_position(800.0, 600.0, Vec2::new(200.0, 100.0));
        assert!((pos.x - 600.0).abs() < f32::EPSILON);
        assert!((pos.y - 500.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_text_start_position() {
        let overlay = DebugOverlay::new();
        let bg_size = Vec2::new(200.0, 100.0);
        let pos = overlay.text_start_position(800.0, 600.0, bg_size);
        assert!((pos.x - overlay.padding).abs() < f32::EPSILON);
        assert!((pos.y - overlay.padding).abs() < f32::EPSILON);
    }

    #[test]
    fn test_custom_colors() {
        let mut overlay = DebugOverlay::new();
        overlay.text_color = Color::YELLOW;
        overlay.bg_color = Color::new(0.1, 0.1, 0.1, 0.9);
        assert_eq!(overlay.text_color, Color::YELLOW);
        assert_eq!(overlay.bg_color.a, 0.9);
    }

    #[test]
    fn test_disable_all_sections() {
        let mut overlay = DebugOverlay::new();
        overlay.show_fps = false;
        overlay.show_frame_time = false;
        overlay.show_entity_count = false;
        overlay.show_profiler = false;
        let lines = overlay.text_lines();
        assert!(lines.is_empty());
    }
}
