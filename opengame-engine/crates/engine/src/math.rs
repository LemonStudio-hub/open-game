pub use glam::{Mat3, Mat4, Vec2, Vec3, Vec4};

pub const PI: f32 = std::f32::consts::PI;
pub const TAU: f32 = std::f32::consts::TAU;
pub const DEG_TO_RAD: f32 = PI / 180.0;
pub const RAD_TO_DEG: f32 = 180.0 / PI;

#[inline]
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * DEG_TO_RAD
}

#[inline]
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * RAD_TO_DEG
}

#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[inline]
pub fn lerp_vec2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    a + (b - a) * t
}

#[inline]
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[inline]
pub fn inverse_lerp(a: f32, b: f32, value: f32) -> f32 {
    if (b - a).abs() < f32::EPSILON {
        0.0
    } else {
        (value - a) / (b - a)
    }
}

#[inline]
pub fn remap(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    let t = inverse_lerp(from_min, from_max, value);
    lerp(to_min, to_max, t)
}

#[inline]
pub fn vec2_angle(a: Vec2, b: Vec2) -> f32 {
    (a.y - b.y).atan2(a.x - b.x)
}

#[inline]
pub fn vec2_from_angle(angle: f32) -> Vec2 {
    Vec2::new(angle.cos(), angle.sin())
}

#[inline]
pub fn ortho_matrix(left: f32, right: f32, bottom: f32, top: f32) -> Mat4 {
    Mat4::orthographic_rh_gl(left, right, bottom, top, -1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lerp() {
        assert!((lerp(0.0, 10.0, 0.5) - 5.0).abs() < f32::EPSILON);
        assert!((lerp(0.0, 10.0, 0.0) - 0.0).abs() < f32::EPSILON);
        assert!((lerp(0.0, 10.0, 1.0) - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_lerp_negative() {
        assert!((lerp(-10.0, 10.0, 0.5) - 0.0).abs() < f32::EPSILON);
        assert!((lerp(-5.0, -1.0, 0.5) - (-3.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_lerp_vec2() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, 20.0);
        let result = lerp_vec2(a, b, 0.5);
        assert!((result.x - 5.0).abs() < f32::EPSILON);
        assert!((result.y - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_deg_rad() {
        assert!((deg_to_rad(180.0) - PI).abs() < f32::EPSILON);
        assert!((rad_to_deg(PI) - 180.0).abs() < f32::EPSILON);
        assert!((deg_to_rad(90.0) - PI / 2.0).abs() < f32::EPSILON);
        assert!((deg_to_rad(360.0) - TAU).abs() < f32::EPSILON);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-1.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn test_smoothstep() {
        assert!((smoothstep(0.0, 1.0, 0.0) - 0.0).abs() < f32::EPSILON);
        assert!((smoothstep(0.0, 1.0, 1.0) - 1.0).abs() < f32::EPSILON);
        assert!((smoothstep(0.0, 1.0, 0.5) - 0.5).abs() < f32::EPSILON);
        let mid = smoothstep(0.0, 1.0, 0.25);
        assert!(mid > 0.0 && mid < 0.25);
    }

    #[test]
    fn test_inverse_lerp() {
        assert!((inverse_lerp(0.0, 10.0, 5.0) - 0.5).abs() < f32::EPSILON);
        assert!((inverse_lerp(0.0, 10.0, 0.0) - 0.0).abs() < f32::EPSILON);
        assert!((inverse_lerp(0.0, 10.0, 10.0) - 1.0).abs() < f32::EPSILON);
        assert_eq!(inverse_lerp(5.0, 5.0, 3.0), 0.0);
    }

    #[test]
    fn test_remap() {
        let result = remap(5.0, 0.0, 10.0, 0.0, 100.0);
        assert!((result - 50.0).abs() < f32::EPSILON);

        let result = remap(0.0, -1.0, 1.0, 0.0, 255.0);
        assert!((result - 127.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vec2_angle() {
        let angle = vec2_angle(Vec2::new(1.0, 0.0), Vec2::new(0.0, 0.0));
        assert!((angle - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_vec2_from_angle() {
        let v = vec2_from_angle(0.0);
        assert!((v.x - 1.0).abs() < f32::EPSILON);
        assert!((v.y - 0.0).abs() < f32::EPSILON);

        let v = vec2_from_angle(PI / 2.0);
        assert!((v.x - 0.0).abs() < 1e-6);
        assert!((v.y - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_ortho_matrix() {
        let m = ortho_matrix(0.0, 800.0, 0.0, 600.0);
        assert_ne!(m, Mat4::IDENTITY);
    }
}
