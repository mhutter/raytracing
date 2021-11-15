use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

mod sphere;
pub use sphere::*;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest_so_far = t_max;

        for hittable in self.iter() {
            if let Some(rec) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit = Some(rec);
            }
        }

        hit
    }
}
