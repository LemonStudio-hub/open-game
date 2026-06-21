use crate::math::{ortho_matrix, Mat4, Vec2};
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Camera2D {
    pub position: Vec2,
    pub zoom: f32,
    pub rotation: f32,
    viewport_width: f32,
    viewport_height: f32,
    projection: Cell<Mat4>,
    view: Cell<Mat4>,
    dirty: Cell<bool>,
}

impl Camera2D {
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        let cam = Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            rotation: 0.0,
            viewport_width,
            viewport_height,
            projection: Cell::new(Mat4::IDENTITY),
            view: Cell::new(Mat4::IDENTITY),
            dirty: Cell::new(true),
        };
        cam.update_matrices();
        cam
    }

    pub fn set_viewport(&mut self, width: f32, height: f32) {
        self.viewport_width = width;
        self.viewport_height = height;
        self.dirty.set(true);
    }

    pub fn mark_dirty(&self) {
        self.dirty.set(true);
    }

    pub fn projection(&self) -> Mat4 {
        self.update_matrices();
        self.projection.get()
    }

    pub fn view(&self) -> Mat4 {
        self.update_matrices();
        self.view.get()
    }

    pub fn view_projection(&self) -> Mat4 {
        self.update_matrices();
        self.projection.get() * self.view.get()
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        let half_w = self.viewport_width / 2.0;
        let half_h = self.viewport_height / 2.0;
        let normalized = Vec2::new(
            (screen_pos.x - half_w) / half_w,
            (half_h - screen_pos.y) / half_h,
        );
        normalized / self.zoom + self.position
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        let relative = (world_pos - self.position) * self.zoom;
        Vec2::new(
            relative.x + self.viewport_width / 2.0,
            -relative.y + self.viewport_height / 2.0,
        )
    }

    fn update_matrices(&self) {
        if !self.dirty.get() {
            return;
        }

        let half_w = self.viewport_width / 2.0;
        let half_h = self.viewport_height / 2.0;

        self.projection.set(ortho_matrix(-half_w, half_w, -half_h, half_h));

        let offset = -self.position * self.zoom;
        self.view.set(
            Mat4::from_translation(glam::Vec3::new(offset.x, offset.y, 0.0))
                * Mat4::from_rotation_z(self.rotation)
                * Mat4::from_scale(glam::Vec3::new(self.zoom, self.zoom, 1.0)),
        );

        self.dirty.set(false);
    }
}
