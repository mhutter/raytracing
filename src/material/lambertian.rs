use rand::prelude::SmallRng;

use crate::{hittable::HitRecord, random_unit_vector, ray::Ray, vec3::Color};

use super::{Material, ScatterResult};

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, rec: &HitRecord, rng: &mut SmallRng) -> ScatterResult {
        let scatter_direction = match rec.normal + random_unit_vector(rng) {
            dir if dir.near_zero() => rec.normal,
            dir => dir,
        };

        ScatterResult::Scattered(
            Ray {
                origin: rec.p,
                direction: scatter_direction,
            },
            self.albedo,
        )
    }
}
