use cgmath::{InnerSpace, Vector3};

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    (1.0 - t) * a + t * b
}

pub fn reflect(v: Vector3<f64>, axis: Vector3<f64>) -> Vector3<f64> {
    -v + axis * v.dot(axis) * 2.0
}
