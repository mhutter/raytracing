use std::f64::consts::PI;

use vec3::Point3;

pub mod camera;
pub mod hittable;
pub mod ray;
pub mod vec3;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_in_unit_sphere() -> Point3 {
    loop {
        let p = Point3::new_random_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
