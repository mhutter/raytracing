use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

use super::{Material, ScatterResult};

#[derive(Clone, Copy)]
pub struct Dielectric {
    // Index of refraction
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, rec: &HitRecord) -> ScatterResult {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray.direction.unit_vector();
        let refracted = unit_direction.refract(rec.normal, refraction_ratio);

        ScatterResult::Scattered(
            Ray {
                origin: rec.p,
                direction: refracted,
            },
            Color::new(1, 1, 1),
        )
    }
}
