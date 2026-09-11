// color.rs
use std::ops::{Add, Mul, Sub};

use crate::math;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn red(&self) -> f64 {
        self.r
    }

    pub fn green(&self) -> f64 {
        self.g
    }

    pub fn blue(&self) -> f64 {
        self.b
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        math::equal(self.r, other.r) && math::equal(self.g, other.g) && math::equal(self.b, other.b)
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self::new(self.r * scalar, self.g * scalar, self.b * scalar)
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn hadamard_product() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
