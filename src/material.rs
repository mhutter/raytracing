use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

mod lambertian;
pub use lambertian::*;
mod metal;
pub use metal::*;
mod dielectric;
pub use dielectric::*;

pub enum ScatterResult {
    Absorbed(Ray),
    Scattered(Ray, Color),
}

pub trait Material {
    /// 1. Produce a scattered ray (or say it absorbed the incident ray).
    /// 2. If scattered, say how much the ray should be attenuated.
    fn scatter(&self, ray: Ray, rec: &HitRecord) -> ScatterResult;
}
