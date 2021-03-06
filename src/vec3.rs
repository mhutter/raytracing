use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rand::{distributions::Uniform, prelude::SmallRng, Rng};

use crate::random_in_unit_sphere;

pub mod color;
pub use color::*;
pub mod point3;
pub use point3::*;

/// Vec3 represents a three-dimensional vector
#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    /// Create a new Vec3 from any 3 values that can be converted to f64
    ///
    /// Example:
    /// ```
    /// use raytracing::vec3::Vec3;
    ///
    /// let a = Vec3::new(1, 2, 3);
    /// let b = Vec3::new(1.0, 2.0, 3.0);
    /// let c = Vec3::new(1, 2.0, 3);
    /// assert_eq!(a, b);
    /// assert_eq!(a, b);
    /// assert_eq!(b, c);
    /// ```
    pub fn new<T, U, V>(x: T, y: U, z: V) -> Self
    where
        T: Into<f64>,
        U: Into<f64>,
        V: Into<f64>,
    {
        Self(x.into(), y.into(), z.into())
    }

    /// Create a new random unit vector
    ///
    /// Example:
    /// ```
    /// use raytracing::vec3::Vec3;
    /// use rand::prelude::*;
    ///
    /// let mut rng = SmallRng::from_entropy();
    /// let r = Vec3::new_random(&mut rng);
    /// assert!(r.0 >= 0.0);
    /// assert!(r.0 < 1.0);
    /// assert!(r.1 >= 0.0);
    /// assert!(r.1 < 1.0);
    /// assert!(r.2 >= 0.0);
    /// assert!(r.2 < 1.0);
    /// ```
    pub fn new_random(rng: &mut SmallRng) -> Self {
        Self::new_random_range(rng, 0.0, 1.0)
    }

    /// Create a new random Vec3 where all three values are in the range [0.0, 1.0)
    pub fn new_random_range(rng: &mut SmallRng, min: f64, max: f64) -> Self {
        Self(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn x(self) -> f64 {
        self.0
    }
    pub fn y(self) -> f64 {
        self.1
    }
    pub fn z(self) -> f64 {
        self.2
    }

    /// Get the length of the vector
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Get the square of the length of the vector
    pub fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    /// Calculate the dot product of `self` and `rhs`
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    /// Calculate the cross product of `self` and `rhs`
    pub fn cross(self, rhs: Self) -> Self {
        let Vec3(sx, sy, sz) = self;
        let Vec3(rx, ry, rz) = rhs;

        let x = sy * rz - sz * ry;
        let y = sz * rx - sx * rz;
        let z = sx * ry - sy * rx;

        Self(x, y, z)
    }

    /// Calculate an unit vector (a vector of length 1) pointing in the
    /// same direction as `self`
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    /// Return a random vector pointing in the same hemisphere as "self"
    pub fn random_in_hemisphere(&self, rng: &mut SmallRng) -> Self {
        let in_unit_sphere = random_in_unit_sphere(rng);
        if in_unit_sphere.dot(self) > 0.0 {
            // in the same hemisphere as the normal ("self")
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk(rng: &mut SmallRng) -> Self {
        // Preparing uniform distribution beforehand performs better when
        // multiple values are generated.
        let distr = Uniform::from(-1.0..1.0);

        loop {
            let p = Vec3(rng.sample(distr), rng.sample(distr), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Returns whether all 3 values are below 0.00000001
    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        (self.0.abs() < s) && (self.1.abs() < s) && (self.2.abs() < s)
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2.0 * self.dot(normal) * *normal
    }

    pub fn refract(&self, normal: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *normal);
        let r_out_parallel = (-(1.0 - r_out_perp.length_squared()).abs().sqrt()) * *normal;
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}
impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Vec3(1.0, 2.0, 3.0), Vec3::new(1, 2, 3));
        assert_eq!(Vec3(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(Vec3(1.0, 2.0, 3.0), Vec3::new(1, 2.0, 3));
    }

    #[test]
    fn test_length_squared() {
        assert_eq!(14.0, Vec3::new(1, 2, 3).length_squared());
        assert_eq!(77.0, Vec3::new(4, 5, 6).length_squared());
    }

    #[test]
    fn test_length() {
        assert_eq!(1.7320508075688772, Vec3::new(1, 1, 1).length());
        assert_eq!(8.774964387392123, Vec3::new(4, 5, 6).length());
    }

    #[test]
    fn test_dot() {
        let a = &Vec3::new(1, 2, 3);
        let b = &Vec3::new(4, 5, 6);
        assert_eq!(14.0, a.dot(a));
        assert_eq!(32.0, a.dot(b));
        assert_eq!(77.0, b.dot(b));
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);
        // TODO: Verify those values make sense
        assert_eq!(Vec3(0.0, 0.0, 0.0), a.cross(a));
        assert_eq!(Vec3(-3.0, 6.0, -3.0), a.cross(b));
        assert_eq!(Vec3(0.0, 0.0, 0.0), b.cross(b));
    }

    #[test]
    fn test_unit_vector() {
        // TODO: Verify those values make sense
        assert_eq!(
            Vec3(0.2672612419124244, 0.5345224838248488, 0.8017837257372732),
            Vec3::new(1, 2, 3).unit_vector()
        );
        assert_eq!(
            Vec3(0.4558423058385518, 0.5698028822981898, 0.6837634587578276),
            Vec3::new(4, 5, 6).unit_vector()
        );
    }

    #[test]
    fn test_add() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);
        assert_eq!(Vec3::new(5, 7, 9), a + b);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);
        a += b;
        assert_eq!(Vec3::new(5, 7, 9), a);
    }

    #[test]
    fn test_sub() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);
        assert_eq!(Vec3::new(3, 3, 3), b - a);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);
        a -= b;
        assert_eq!(Vec3::new(-3, -3, -3), a);
    }

    #[test]
    fn test_mul_self() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);
        assert_eq!(Vec3::new(4, 10, 18), a * b);
    }

    #[test]
    fn test_mul_f64() {
        let a = Vec3::new(1, 2, 3);
        assert_eq!(Vec3::new(1.5, 3.0, 4.5), a * 1.5);
        assert_eq!(Vec3::new(1.5, 3.0, 4.5), 1.5 * a);
    }

    #[test]
    fn test_mul_assign_self() {
        let mut a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);
        a *= b;
        assert_eq!(Vec3::new(4, 10, 18), a);
    }

    #[test]
    fn test_mul_assign_f64() {
        let mut a = Vec3::new(1, 2, 3);
        a *= 1.5;
        assert_eq!(Vec3::new(1.5, 3.0, 4.5), a);
    }

    #[test]
    fn test_div_self() {
        let a = Vec3::new(4, 5, 6);
        let b = Vec3::new(1, 2, 3);
        assert_eq!(Vec3::new(4, 2.5, 2), a / b);
    }

    #[test]
    fn test_div_f64() {
        let a = Vec3::new(1, 2, 3);
        assert_eq!(Vec3::new(0.5, 1.0, 1.5), a / 2.0);
    }

    #[test]
    fn test_div_assign_self() {
        let mut a = Vec3::new(4, 5, 6);
        let b = Vec3::new(1, 2, 3);
        a /= b;
        assert_eq!(Vec3::new(4, 2.5, 2), a);
    }

    #[test]
    fn test_div_assign_f64() {
        let mut a = Vec3::new(2, 4, 6);
        a /= 2.0;
        assert_eq!(Vec3::new(1, 2, 3), a);
    }

    #[test]
    fn test_neg() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(4, 5, 6);

        assert_eq!(Vec3::new(-1, -2, -3), -a);
        assert_eq!(Vec3::new(-4, -5, -6), -b);
    }

    #[test]
    fn test_sum() {
        let vec = vec![Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(7, 8, 9)];

        assert_eq!(Vec3::new(12, 15, 18), vec.into_iter().sum());
    }
}
