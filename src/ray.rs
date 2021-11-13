use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let ray = Ray {
            origin: Point3::new(0, 0, 0),
            direction: Vec3::new(1, 2, 3),
        };
        assert_eq!(Point3::new(1, 2, 3), ray.at(1.0));
        assert_eq!(Point3::new(2, 4, 6), ray.at(2.0));

        let ray = Ray {
            origin: Point3::new(1, 1, 1),
            direction: Vec3::new(1, 2, 3),
        };
        assert_eq!(Point3::new(2, 3, 4), ray.at(1.0));
        assert_eq!(Point3::new(3, 5, 7), ray.at(2.0));
    }
}
