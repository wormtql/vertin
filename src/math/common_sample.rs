use std::f64::consts::PI;
use cgmath::Vector2;

/// Sample a uniform disk with radius = 1. Uniformly samples (r, θ)
/// The disk centers at XY plane (0, 0) and spans a radius = 1 circle
pub fn sample_uniform_disk_polar(r1: f64, r2: f64) -> Vector2<f64> {
    let theta = 2.0 * PI * r2;
    Vector2::new(r1 * theta.cos(), r1 * theta.sin())
}
