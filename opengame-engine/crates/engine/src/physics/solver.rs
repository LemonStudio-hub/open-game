use super::collision::CollisionInfo;
use super::rigid_body::RigidBody;
use crate::math::Vec2;

pub struct Solver {
    pub gravity: Vec2,
    pub position_iterations: u32,
    pub velocity_iterations: u32,
    pub correction_factor: f32,
}

impl Solver {
    pub fn new(gravity: Vec2) -> Self {
        Self {
            gravity,
            position_iterations: 4,
            velocity_iterations: 4,
            correction_factor: 0.2,
        }
    }

    pub fn apply_gravity(&self, body: &mut RigidBody) {
        if !body.is_dynamic() {
            return;
        }
        // Gravity is an acceleration (not a force), so do not multiply by mass.
        body.velocity += self.gravity * body.gravity_scale;
    }

    pub fn integrate(&self, body: &mut RigidBody, position: &mut Vec2, dt: f32) {
        if body.is_static() {
            return;
        }

        if body.is_dynamic() {
            body.velocity += body.force * body.inv_mass() * dt;
            body.velocity *= 1.0 / (1.0 + body.linear_damping * dt);
        }

        *position += body.velocity * dt;
        body.force = Vec2::ZERO;
    }

    pub fn resolve_collision(
        &self,
        info: &CollisionInfo,
        body_a: &mut RigidBody,
        pos_a: &mut Vec2,
        body_b: &mut RigidBody,
        pos_b: &mut Vec2,
    ) {
        let inv_mass_a = body_a.inv_mass();
        let inv_mass_b = body_b.inv_mass();
        let total_inv_mass = inv_mass_a + inv_mass_b;

        if total_inv_mass <= 0.0 {
            return;
        }

        // Position correction (Baumgarte stabilization)
        let separation = info.normal * info.depth * self.correction_factor;
        let ratio_a = inv_mass_a / total_inv_mass;
        let ratio_b = inv_mass_b / total_inv_mass;

        *pos_a -= separation * ratio_a;
        *pos_b += separation * ratio_b;

        // Velocity resolution
        let relative_vel = body_b.velocity - body_a.velocity;
        let vel_along_normal = relative_vel.dot(info.normal);

        if vel_along_normal > 0.0 {
            return;
        }

        let restitution = (info.restitution_a + info.restitution_b) * 0.5;
        let impulse_scalar = -(1.0 + restitution) * vel_along_normal / total_inv_mass;
        let impulse = info.normal * impulse_scalar;

        body_a.velocity -= impulse * inv_mass_a;
        body_b.velocity += impulse * inv_mass_b;

        // Friction impulse
        let tangent_vel = relative_vel - info.normal * vel_along_normal;
        let tangent_len_sq = tangent_vel.length_squared();
        if tangent_len_sq > f32::EPSILON {
            let tangent = tangent_vel.normalize();
            let friction = (info.friction_a + info.friction_b) * 0.5;
            let jt = -tangent_vel.dot(tangent) / total_inv_mass;
            let friction_impulse = if jt.abs() < impulse_scalar.abs() * friction {
                tangent * jt
            } else {
                tangent * (-impulse_scalar * friction)
            };

            body_a.velocity -= friction_impulse * inv_mass_a;
            body_b.velocity += friction_impulse * inv_mass_b;
        }
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new(Vec2::new(0.0, 980.0))
    }
}
