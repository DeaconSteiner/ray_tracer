// math.rs
const EPSILON: f64 = 1e-5;

pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_within_epsilon_are_equal() {
        assert!(equal(1.0, 1.000001));
    }

    #[test]
    fn numbers_outside_epsilon_are_not_equal() {
        assert!(!equal(1.0, 1.1));
    }

    #[test]
    fn comparison_is_symmetric() {
        assert!(equal(1.0, 1.000001));
        assert!(equal(1.000001, 1.0));
    }
}
