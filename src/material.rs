use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

mod lambertian;
pub use lambertian::*;

pub trait Material {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> (Color, Ray);
}
