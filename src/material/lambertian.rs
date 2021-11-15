use crate::{hittable::HitRecord, random_unit_vector, ray::Ray, vec3::Color};

use super::Material;

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
    fn scatter(&self, _ray: Ray, rec: HitRecord) -> (Color, Ray) {
        let scatter_direction = match rec.normal + random_unit_vector() {
            dir if dir.near_zero() => rec.normal,
            dir => dir,
        };

        (
            self.albedo,
            Ray {
                origin: rec.p,
                direction: scatter_direction,
            },
        )
    }
}
