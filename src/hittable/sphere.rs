use crate::{
    hittable::HitRecord,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::Hittable;

pub struct Sphere<M: Material> {
    pub center: Point3,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: 'static + Material + Copy> Hittable for Sphere<M> {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            // did not hit
            return None;
        }

        // Find nearset root that lies in acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        // We have a hit
        let mut hit = HitRecord {
            t: root,
            p: ray.at(root),
            material: Box::new(self.material),
            normal: Vec3::default(),
            front_face: true,
        };
        let outward_normal = (hit.p - self.center) / self.radius;
        hit.set_face_normal(ray, outward_normal);

        Some(hit)
    }
}
