use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

use super::Material;
pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, rec: &HitRecord) -> (Ray, Option<Color>) {
        let reflected = ray.direction.reflect(rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected,
        };

        match scattered.direction.dot(rec.normal) {
            x if x > 0.0 => (scattered, Some(self.albedo)),
            _ => (scattered, None),
        }
    }
}
