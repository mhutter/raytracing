use rand::{prelude::SmallRng, Rng};

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
    fn scatter(&self, ray: Ray, rec: &HitRecord, rng: &mut SmallRng) -> ScatterResult {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray.direction.unit_vector();
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen() {
            unit_direction.reflect(&rec.normal)
        } else {
            unit_direction.refract(&rec.normal, refraction_ratio)
        };

        ScatterResult::Scattered(
            Ray {
                origin: rec.p,
                direction,
            },
            Color::new(1, 1, 1),
        )
    }
}

/// Calculate reflectance using Schlick's approximation
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
