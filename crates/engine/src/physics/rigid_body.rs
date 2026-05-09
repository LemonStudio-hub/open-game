use crate::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BodyType {
    Dynamic,
    Kinematic,
    Static,
}

#[derive(Debug, Clone)]
pub struct RigidBody {
    pub body_type: BodyType,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub gravity_scale: f32,
    pub angular_velocity: f32,
    pub angular_damping: f32,
    pub linear_damping: f32,
    pub(crate) force: Vec2,
}

impl RigidBody {
    pub fn dynamic() -> Self {
        Self {
            body_type: BodyType::Dynamic,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass: 1.0,
            gravity_scale: 1.0,
            angular_velocity: 0.0,
            angular_damping: 0.0,
            linear_damping: 0.0,
            force: Vec2::ZERO,
        }
    }

    pub fn kinematic() -> Self {
        Self {
            body_type: BodyType::Kinematic,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass: f32::INFINITY,
            gravity_scale: 0.0,
            angular_velocity: 0.0,
            angular_damping: 0.0,
            linear_damping: 0.0,
            force: Vec2::ZERO,
        }
    }

    pub fn static_body() -> Self {
        Self {
            body_type: BodyType::Static,
            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,
            mass: f32::INFINITY,
            gravity_scale: 0.0,
            angular_velocity: 0.0,
            angular_damping: 0.0,
            linear_damping: 0.0,
            force: Vec2::ZERO,
        }
    }

    pub fn with_mass(mut self, mass: f32) -> Self {
        self.mass = mass;
        self
    }

    pub fn with_gravity_scale(mut self, scale: f32) -> Self {
        self.gravity_scale = scale;
        self
    }

    pub fn with_velocity(mut self, velocity: Vec2) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.force += force;
    }

    pub fn apply_impulse(&mut self, impulse: Vec2) {
        if self.mass.is_finite() && self.mass > 0.0 {
            self.velocity += impulse / self.mass;
        }
    }

    pub fn is_dynamic(&self) -> bool {
        self.body_type == BodyType::Dynamic
    }

    pub fn is_static(&self) -> bool {
        self.body_type == BodyType::Static
    }

    pub fn is_kinematic(&self) -> bool {
        self.body_type == BodyType::Kinematic
    }

    pub fn inv_mass(&self) -> f32 {
        if self.mass.is_infinite() || self.mass <= 0.0 {
            0.0
        } else {
            1.0 / self.mass
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_body() {
        let body = RigidBody::dynamic();
        assert_eq!(body.body_type, BodyType::Dynamic);
        assert_eq!(body.mass, 1.0);
        assert!(body.is_dynamic());
        assert!(!body.is_static());
        assert!(!body.is_kinematic());
    }

    #[test]
    fn test_static_body() {
        let body = RigidBody::static_body();
        assert_eq!(body.body_type, BodyType::Static);
        assert!(body.mass.is_infinite());
        assert!(body.is_static());
        assert!(!body.is_dynamic());
    }

    #[test]
    fn test_kinematic_body() {
        let body = RigidBody::kinematic();
        assert_eq!(body.body_type, BodyType::Kinematic);
        assert!(body.is_kinematic());
        assert!(!body.is_dynamic());
    }

    #[test]
    fn test_with_mass() {
        let body = RigidBody::dynamic().with_mass(5.0);
        assert_eq!(body.mass, 5.0);
    }

    #[test]
    fn test_with_gravity_scale() {
        let body = RigidBody::dynamic().with_gravity_scale(2.0);
        assert_eq!(body.gravity_scale, 2.0);
    }

    #[test]
    fn test_with_velocity() {
        let body = RigidBody::dynamic().with_velocity(Vec2::new(10.0, 5.0));
        assert_eq!(body.velocity, Vec2::new(10.0, 5.0));
    }

    #[test]
    fn test_apply_force() {
        let mut body = RigidBody::dynamic();
        body.apply_force(Vec2::new(10.0, 0.0));
        body.apply_force(Vec2::new(5.0, 5.0));
        assert_eq!(body.force, Vec2::new(15.0, 5.0));
    }

    #[test]
    fn test_apply_impulse() {
        let mut body = RigidBody::dynamic().with_mass(2.0);
        body.apply_impulse(Vec2::new(10.0, 0.0));
        assert!((body.velocity.x - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_apply_impulse_static() {
        let mut body = RigidBody::static_body();
        body.apply_impulse(Vec2::new(100.0, 100.0));
        assert_eq!(body.velocity, Vec2::ZERO);
    }

    #[test]
    fn test_inv_mass_dynamic() {
        let body = RigidBody::dynamic().with_mass(4.0);
        assert!((body.inv_mass() - 0.25).abs() < f32::EPSILON);
    }

    #[test]
    fn test_inv_mass_static() {
        let body = RigidBody::static_body();
        assert_eq!(body.inv_mass(), 0.0);
    }

    #[test]
    fn test_inv_mass_zero() {
        let body = RigidBody::dynamic().with_mass(0.0);
        assert_eq!(body.inv_mass(), 0.0);
    }
}
