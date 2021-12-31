use rand::prelude::SmallRng;

use crate::{hittable::HitRecord, random_in_unit_sphere, ray::Ray, vec3::Color};

use super::{Material, ScatterResult};

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, rec: &HitRecord, rng: &mut SmallRng) -> ScatterResult {
        let reflected = ray.direction.reflect(&rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * random_in_unit_sphere(rng),
        };

        match scattered.direction.dot(&rec.normal) {
            x if x > 0.0 => ScatterResult::Scattered(scattered, self.albedo),
            _ => ScatterResult::Absorbed(scattered),
        }
    }
}
