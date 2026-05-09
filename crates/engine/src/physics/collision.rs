use crate::math::Vec2;
use crate::ecs::entity::Entity;
use super::collider::{Collider, ColliderShape};

#[derive(Debug, Clone)]
pub struct CollisionInfo {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub normal: Vec2,
    pub depth: f32,
    pub point: Vec2,
}

impl CollisionInfo {
    pub fn new(entity_a: Entity, entity_b: Entity, normal: Vec2, depth: f32) -> Self {
        Self {
            entity_a,
            entity_b,
            normal,
            depth,
            point: Vec2::ZERO,
        }
    }
}

pub fn test_aabb_aabb(
    pos_a: Vec2, half_a: Vec2,
    pos_b: Vec2, half_b: Vec2,
) -> Option<(Vec2, f32)> {
    let dx = pos_b.x - pos_a.x;
    let dy = pos_b.y - pos_a.y;
    let ox = (half_a.x + half_b.x) - dx.abs();
    let oy = (half_a.y + half_b.y) - dy.abs();

    if ox <= 0.0 || oy <= 0.0 {
        return None;
    }

    if ox < oy {
        let normal = if dx < 0.0 { Vec2::NEG_X } else { Vec2::X };
        Some((normal, ox))
    } else {
        let normal = if dy < 0.0 { Vec2::NEG_Y } else { Vec2::Y };
        Some((normal, oy))
    }
}

pub fn test_circle_circle(
    pos_a: Vec2, radius_a: f32,
    pos_b: Vec2, radius_b: f32,
) -> Option<(Vec2, f32)> {
    let diff = pos_b - pos_a;
    let dist_sq = diff.length_squared();
    let radius_sum = radius_a + radius_b;

    if dist_sq >= radius_sum * radius_sum {
        return None;
    }

    let dist = dist_sq.sqrt();
    if dist < f32::EPSILON {
        Some((Vec2::X, radius_sum))
    } else {
        let normal = diff / dist;
        let depth = radius_sum - dist;
        Some((normal, depth))
    }
}

pub fn test_aabb_circle(
    aabb_pos: Vec2, aabb_half: Vec2,
    circle_pos: Vec2, circle_radius: f32,
) -> Option<(Vec2, f32)> {
    let diff = circle_pos - aabb_pos;
    let clamped = Vec2::new(
        diff.x.clamp(-aabb_half.x, aabb_half.x),
        diff.y.clamp(-aabb_half.y, aabb_half.y),
    );

    let closest = aabb_pos + clamped;
    let to_circle = circle_pos - closest;
    let dist_sq = to_circle.length_squared();

    if dist_sq >= circle_radius * circle_radius {
        return None;
    }

    let dist = dist_sq.sqrt();
    if dist < f32::EPSILON {
        if diff.x.abs() > diff.y.abs() {
            let normal = if diff.x > 0.0 { Vec2::X } else { Vec2::NEG_X };
            Some((normal, aabb_half.x + circle_radius))
        } else {
            let normal = if diff.y > 0.0 { Vec2::Y } else { Vec2::NEG_Y };
            Some((normal, aabb_half.y + circle_radius))
        }
    } else {
        let normal = to_circle / dist;
        let depth = circle_radius - dist;
        Some((normal, depth))
    }
}

pub fn test_collision(
    pos_a: Vec2, collider_a: &Collider,
    pos_b: Vec2, collider_b: &Collider,
) -> Option<(Vec2, f32)> {
    let center_a = pos_a + collider_a.offset;
    let center_b = pos_b + collider_b.offset;

    if (collider_a.layer & collider_b.mask) == 0 || (collider_b.layer & collider_a.mask) == 0 {
        return None;
    }

    match (&collider_a.shape, &collider_b.shape) {
        (ColliderShape::Rectangle { width: w1, height: h1 },
         ColliderShape::Rectangle { width: w2, height: h2 }) => {
            let half_a = Vec2::new(*w1 * 0.5, *h1 * 0.5);
            let half_b = Vec2::new(*w2 * 0.5, *h2 * 0.5);
            test_aabb_aabb(center_a, half_a, center_b, half_b)
        }
        (ColliderShape::Circle { radius: r1 },
         ColliderShape::Circle { radius: r2 }) => {
            test_circle_circle(center_a, *r1, center_b, *r2)
        }
        (ColliderShape::Rectangle { width, height },
         ColliderShape::Circle { radius }) => {
            let half = Vec2::new(*width * 0.5, *height * 0.5);
            test_aabb_circle(center_a, half, center_b, *radius)
        }
        (ColliderShape::Circle { radius },
         ColliderShape::Rectangle { width, height }) => {
            let half = Vec2::new(*width * 0.5, *height * 0.5);
            test_aabb_circle(center_b, half, center_a, *radius).map(|(n, d)| (-n, d))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_overlap() {
        let result = test_aabb_aabb(
            Vec2::ZERO, Vec2::new(1.0, 1.0),
            Vec2::new(1.5, 0.0), Vec2::new(1.0, 1.0),
        );
        assert!(result.is_some());
        let (normal, depth) = result.unwrap();
        assert!((depth - 0.5).abs() < f32::EPSILON);
        assert!((normal.x - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_aabb_no_overlap() {
        let result = test_aabb_aabb(
            Vec2::ZERO, Vec2::new(1.0, 1.0),
            Vec2::new(5.0, 5.0), Vec2::new(1.0, 1.0),
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_aabb_touching() {
        let result = test_aabb_aabb(
            Vec2::ZERO, Vec2::new(1.0, 1.0),
            Vec2::new(2.0, 0.0), Vec2::new(1.0, 1.0),
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_circle_overlap() {
        let result = test_circle_circle(
            Vec2::ZERO, 2.0,
            Vec2::new(3.0, 0.0), 2.0,
        );
        assert!(result.is_some());
        let (_normal, depth) = result.unwrap();
        assert!((depth - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_circle_no_overlap() {
        let result = test_circle_circle(
            Vec2::ZERO, 1.0,
            Vec2::new(5.0, 0.0), 1.0,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_circle_concentric() {
        let result = test_circle_circle(
            Vec2::ZERO, 2.0,
            Vec2::ZERO, 2.0,
        );
        assert!(result.is_some());
        let (_normal, depth) = result.unwrap();
        assert!((depth - 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_aabb_circle_overlap() {
        let result = test_aabb_circle(
            Vec2::ZERO, Vec2::new(1.0, 1.0),
            Vec2::new(1.5, 0.0), 1.0,
        );
        assert!(result.is_some());
    }

    #[test]
    fn test_aabb_circle_no_overlap() {
        let result = test_aabb_circle(
            Vec2::ZERO, Vec2::new(1.0, 1.0),
            Vec2::new(5.0, 0.0), 1.0,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_collision_layer_mask() {
        let col_a = Collider::rectangle(2.0, 2.0).with_layer(1).with_mask(2);
        let col_b = Collider::rectangle(2.0, 2.0).with_layer(2).with_mask(1);

        let result = test_collision(Vec2::ZERO, &col_a, Vec2::new(0.5, 0.0), &col_b);
        assert!(result.is_some());
    }

    #[test]
    fn test_collision_layer_mask_filtered() {
        let col_a = Collider::rectangle(2.0, 2.0).with_layer(1).with_mask(4);
        let col_b = Collider::rectangle(2.0, 2.0).with_layer(2).with_mask(8);

        let result = test_collision(Vec2::ZERO, &col_a, Vec2::new(0.5, 0.0), &col_b);
        assert!(result.is_none());
    }

    #[test]
    fn test_collision_with_offset() {
        let col_a = Collider::rectangle(4.0, 4.0).with_offset(Vec2::new(0.5, 0.0));
        let col_b = Collider::rectangle(4.0, 4.0);

        let result = test_collision(Vec2::ZERO, &col_a, Vec2::new(3.0, 0.0), &col_b);
        assert!(result.is_some());
    }

    #[test]
    fn test_collision_circle_rect() {
        let circle = Collider::circle(1.0);
        let rect = Collider::rectangle(2.0, 2.0);

        let result = test_collision(Vec2::new(1.5, 0.0), &circle, Vec2::ZERO, &rect);
        assert!(result.is_some());
    }

    #[test]
    fn test_collision_rect_circle() {
        let rect = Collider::rectangle(2.0, 2.0);
        let circle = Collider::circle(1.0);

        let result = test_collision(Vec2::ZERO, &rect, Vec2::new(1.5, 0.0), &circle);
        assert!(result.is_some());
    }
}
