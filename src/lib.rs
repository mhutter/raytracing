use std::f64::consts::PI;

use rand::{prelude::SmallRng, Rng};
use vec3::{Point3, Vec3};

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod vec3;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_in_unit_sphere(rng: &mut SmallRng) -> Point3 {
    loop {
        let p: Point3 = Vec3(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector(rng: &mut SmallRng) -> Vec3 {
    random_in_unit_sphere(rng).unit_vector()
}
