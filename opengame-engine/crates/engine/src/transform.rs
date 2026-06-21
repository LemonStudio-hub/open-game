use crate::math::{Mat3, Vec2};
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Transform2D {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
    dirty: Cell<bool>,
    local_matrix: Cell<Mat3>,
    world_matrix: Cell<Mat3>,
}

impl Transform2D {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            rotation: 0.0,
            scale: Vec2::ONE,
            dirty: Cell::new(true),
            local_matrix: Cell::new(Mat3::IDENTITY),
            world_matrix: Cell::new(Mat3::IDENTITY),
        }
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self.dirty.set(true);
        self
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self.dirty.set(true);
        self
    }

    pub fn with_uniform_scale(mut self, scale: f32) -> Self {
        self.scale = Vec2::splat(scale);
        self.dirty.set(true);
        self
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.dirty.set(true);
    }

    pub fn translate(&mut self, offset: Vec2) {
        self.position += offset;
        self.dirty.set(true);
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
        self.dirty.set(true);
    }

    pub fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
        self.dirty.set(true);
    }

    pub fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
        self.dirty.set(true);
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.get()
    }

    pub fn update_matrix(&self) {
        if !self.dirty.get() {
            return;
        }

        let cos = self.rotation.cos();
        let sin = self.rotation.sin();

        let sx = self.scale.x;
        let sy = self.scale.y;
        let tx = self.position.x;
        let ty = self.position.y;

        self.local_matrix.set(Mat3::from_cols_array(&[
            cos * sx,
            sin * sx,
            0.0,
            -sin * sy,
            cos * sy,
            0.0,
            tx,
            ty,
            1.0,
        ]));

        self.world_matrix.set(self.local_matrix.get());
        self.dirty.set(false);
    }

    pub fn local_matrix(&self) -> Mat3 {
        self.update_matrix();
        self.local_matrix.get()
    }

    pub fn world_matrix(&self) -> Mat3 {
        self.update_matrix();
        self.world_matrix.get()
    }

    pub fn forward(&self) -> Vec2 {
        Vec2::new(self.rotation.cos(), self.rotation.sin())
    }

    pub fn right(&self) -> Vec2 {
        Vec2::new(-self.rotation.sin(), self.rotation.cos())
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::new(Vec2::ZERO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let t = Transform2D::new(Vec2::new(10.0, 20.0));
        assert_eq!(t.position, Vec2::new(10.0, 20.0));
        assert_eq!(t.rotation, 0.0);
        assert_eq!(t.scale, Vec2::ONE);
        assert!(t.is_dirty());
    }

    #[test]
    fn test_default() {
        let t = Transform2D::default();
        assert_eq!(t.position, Vec2::ZERO);
    }

    #[test]
    fn test_builder_pattern() {
        let t = Transform2D::new(Vec2::ZERO)
            .with_rotation(1.5)
            .with_scale(Vec2::new(2.0, 3.0));
        assert_eq!(t.rotation, 1.5);
        assert_eq!(t.scale, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_with_uniform_scale() {
        let t = Transform2D::new(Vec2::ZERO).with_uniform_scale(5.0);
        assert_eq!(t.scale, Vec2::new(5.0, 5.0));
    }

    #[test]
    fn test_set_position() {
        let mut t = Transform2D::new(Vec2::ZERO);
        t.set_position(Vec2::new(5.0, 10.0));
        assert_eq!(t.position, Vec2::new(5.0, 10.0));
        assert!(t.is_dirty());
    }

    #[test]
    fn test_translate() {
        let mut t = Transform2D::new(Vec2::new(1.0, 2.0));
        t.translate(Vec2::new(3.0, 4.0));
        assert_eq!(t.position, Vec2::new(4.0, 6.0));
    }

    #[test]
    fn test_set_rotation() {
        let mut t = Transform2D::new(Vec2::ZERO);
        t.set_rotation(std::f32::consts::PI);
        assert!((t.rotation - std::f32::consts::PI).abs() < f32::EPSILON);
    }

    #[test]
    fn test_rotate() {
        let mut t = Transform2D::new(Vec2::ZERO);
        t.rotate(1.0);
        t.rotate(2.0);
        assert!((t.rotation - 3.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_set_scale() {
        let mut t = Transform2D::new(Vec2::ZERO);
        t.set_scale(Vec2::new(2.0, 3.0));
        assert_eq!(t.scale, Vec2::new(2.0, 3.0));
    }

    #[test]
    fn test_dirty_flag() {
        let t = Transform2D::new(Vec2::ZERO);
        assert!(t.is_dirty());
        t.update_matrix();
        assert!(!t.is_dirty());
        let _ = t.local_matrix();
        assert!(!t.is_dirty());
    }

    #[test]
    fn test_matrix_identity() {
        let t = Transform2D::new(Vec2::ZERO);
        let m = t.local_matrix();
        assert_eq!(m, Mat3::IDENTITY);
    }

    #[test]
    fn test_matrix_translation() {
        let t = Transform2D::new(Vec2::new(10.0, 20.0));
        let m = t.local_matrix();
        assert!((m.col(2).x - 10.0).abs() < f32::EPSILON);
        assert!((m.col(2).y - 20.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_forward() {
        let t = Transform2D::new(Vec2::ZERO);
        let fwd = t.forward();
        assert!((fwd.x - 1.0).abs() < f32::EPSILON);
        assert!((fwd.y - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_right() {
        let t = Transform2D::new(Vec2::ZERO);
        let right = t.right();
        assert!((right.x - 0.0).abs() < f32::EPSILON);
        assert!((right.y - 1.0).abs() < f32::EPSILON);
    }
}
