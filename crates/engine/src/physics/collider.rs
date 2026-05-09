use crate::math::Vec2;

#[derive(Debug, Clone)]
pub enum ColliderShape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
}

#[derive(Debug, Clone)]
pub struct Collider {
    pub shape: ColliderShape,
    pub offset: Vec2,
    pub is_trigger: bool,
    pub friction: f32,
    pub restitution: f32,
    pub layer: u32,
    pub mask: u32,
}

impl Collider {
    pub fn rectangle(width: f32, height: f32) -> Self {
        Self {
            shape: ColliderShape::Rectangle { width, height },
            offset: Vec2::ZERO,
            is_trigger: false,
            friction: 0.0,
            restitution: 1.0,
            layer: 1,
            mask: u32::MAX,
        }
    }

    pub fn circle(radius: f32) -> Self {
        Self {
            shape: ColliderShape::Circle { radius },
            offset: Vec2::ZERO,
            is_trigger: false,
            friction: 0.0,
            restitution: 1.0,
            layer: 1,
            mask: u32::MAX,
        }
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_trigger(mut self, is_trigger: bool) -> Self {
        self.is_trigger = is_trigger;
        self
    }

    pub fn with_restitution(mut self, restitution: f32) -> Self {
        self.restitution = restitution;
        self
    }

    pub fn with_layer(mut self, layer: u32) -> Self {
        self.layer = layer;
        self
    }

    pub fn with_mask(mut self, mask: u32) -> Self {
        self.mask = mask;
        self
    }

    pub fn half_extents(&self) -> Vec2 {
        match &self.shape {
            ColliderShape::Rectangle { width, height } => Vec2::new(*width * 0.5, *height * 0.5),
            ColliderShape::Circle { radius } => Vec2::splat(*radius),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_collider() {
        let c = Collider::rectangle(10.0, 20.0);
        assert_eq!(c.half_extents(), Vec2::new(5.0, 10.0));
        assert!(!c.is_trigger);
        assert_eq!(c.layer, 1);
        assert_eq!(c.mask, u32::MAX);
        assert_eq!(c.restitution, 1.0);
    }

    #[test]
    fn test_circle_collider() {
        let c = Collider::circle(5.0);
        assert_eq!(c.half_extents(), Vec2::new(5.0, 5.0));
    }

    #[test]
    fn test_with_offset() {
        let c = Collider::rectangle(2.0, 2.0).with_offset(Vec2::new(3.0, 4.0));
        assert_eq!(c.offset, Vec2::new(3.0, 4.0));
    }

    #[test]
    fn test_with_trigger() {
        let c = Collider::rectangle(2.0, 2.0).with_trigger(true);
        assert!(c.is_trigger);
    }

    #[test]
    fn test_with_restitution() {
        let c = Collider::circle(1.0).with_restitution(0.5);
        assert_eq!(c.restitution, 0.5);
    }

    #[test]
    fn test_with_layer() {
        let c = Collider::rectangle(2.0, 2.0).with_layer(4);
        assert_eq!(c.layer, 4);
    }

    #[test]
    fn test_with_mask() {
        let c = Collider::circle(1.0).with_mask(0xFF);
        assert_eq!(c.mask, 0xFF);
    }

    #[test]
    fn test_builder_chaining() {
        let c = Collider::rectangle(10.0, 10.0)
            .with_offset(Vec2::ONE)
            .with_trigger(true)
            .with_restitution(0.8)
            .with_layer(2)
            .with_mask(4);

        assert_eq!(c.offset, Vec2::ONE);
        assert!(c.is_trigger);
        assert_eq!(c.restitution, 0.8);
        assert_eq!(c.layer, 2);
        assert_eq!(c.mask, 4);
    }
}
