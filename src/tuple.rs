// tuple.rs
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::math;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        self.w
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w))
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();

        *self / mag
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Self) -> Self {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        math::equal(self.x, other.x)
            && math::equal(self.y, other.y)
            && math::equal(self.z, other.z)
            && math::equal(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // General Invariants
    #[test]
    fn tuple_with_w_not_1_not_0_is_neither_point_nor_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 2.0);

        assert!(!a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_1_is_point() {
        let a = Tuple::point(4.3, -4.2, 3.1);

        assert_eq!(a, Tuple::new(4.3, -4.2, 3.1, 1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_vector() {
        let a = Tuple::vector(4.3, -4.2, 3.1);

        assert_eq!(a, Tuple::new(4.3, -4.2, 3.1, 0.0));
        assert!(a.is_vector());
        assert!(!a.is_point());
    }

    // Tolerance
    #[test]
    fn tuples_are_equal_with_small_floating_point_differences() {
        let a = Tuple::new(0.1 + 0.2, 0.0, 0.0, 1.0);
        let b = Tuple::new(0.3, 0.0, 0.0, 1.0);

        assert_eq!(a, b);
    }

    // Arithmetic
    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        let sum = a1 + a2;
        assert_eq!(sum, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn point_plus_vector_equals_point() {
        let a1 = Tuple::point(3.0, -2.0, 5.0);
        let a2 = Tuple::vector(-2.0, 3.0, 1.0);

        let sum = a1 + a2;
        assert!(sum.is_point());
    }

    #[test]
    fn vector_plus_vector_equals_vector() {
        let a1 = Tuple::vector(3.0, -2.0, 5.0);
        let a2 = Tuple::vector(-2.0, 3.0, 1.0);

        let sum = a1 + a2;
        assert!(sum.is_vector());
    }

    #[test]
    fn subtracting_a_point_from_a_point() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        let diff = p1 - p2;
        assert_eq!(diff, Tuple::vector(-2.0, -4.0, -6.0));
        assert!(diff.is_vector())
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        let diff = p - v;

        assert_eq!(diff, Tuple::point(-2.0, -4.0, -6.0));
        assert!(diff.is_point())
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        let diff = v1 - v2;

        assert_eq!(diff, Tuple::vector(-2.0, -4.0, -6.0));
        assert!(diff.is_vector())
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        let neg = zero - v;
        assert_eq!(neg, Tuple::vector(-1.0, 2.0, -3.0));
        assert!(neg.is_vector())
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    // Magnitude
    #[test]
    fn computing_magnitude_of_vector() {
        let v1 = Tuple::vector(1.0, 0.0, 0.0);
        let v2 = Tuple::vector(0.0, 1.0, 0.0);
        let v3 = Tuple::vector(0.0, 0.0, 1.0);
        let v4 = Tuple::vector(1.0, 2.0, 3.0);
        let v5 = Tuple::vector(-1.0, -2.0, -3.0);

        assert!(math::equal(v1.magnitude(), 1.0));
        assert!(math::equal(v2.magnitude(), 1.0));
        assert!(math::equal(v3.magnitude(), 1.0));
        assert!(math::equal(v4.magnitude(), f64::sqrt(14.0)));
        assert!(math::equal(v5.magnitude(), f64::sqrt(14.0)));
    }

    #[test]
    fn normalize_vector_4_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);

        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0))
    }

    #[test]
    fn normalize_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(v.normalize(), Tuple::vector(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let norm = v.normalize();

        assert!(math::equal(norm.magnitude(), 1.0))
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert!(math::equal(a.dot(&b), 20.0));
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(a.cross(&b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Tuple::vector(1.0, -2.0, 1.0));
    }
}
