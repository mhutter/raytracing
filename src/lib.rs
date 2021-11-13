use std::f64::consts::PI;

pub mod hittable;
pub mod ray;
pub mod vec3;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
