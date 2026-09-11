// matrix.rs
use crate::{Tuple, math};
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len());
        Self { rows, cols, data }
    }

    pub fn identity(size: usize) -> Self {
        let data = (0..size * size)
            .map(|i| if i / size == i % size { 1.0 } else { 0.0 })
            .collect();
        Self::from_vec(size, size, data)
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Self::from_vec(
            4,
            4,
            vec![
                1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0,
            ],
        )
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Self::from_vec(
            4,
            4,
            vec![
                x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        )
    }

    pub fn rotation_x(radians: f64) -> Self {
        Self::from_vec(
            4,
            4,
            vec![
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                radians.cos(),
                -radians.sin(),
                0.0,
                0.0,
                radians.sin(),
                radians.cos(),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        )
    }

    pub fn rotation_y(radians: f64) -> Self {
        Self::from_vec(
            4,
            4,
            vec![
                radians.cos(),
                0.0,
                radians.sin(),
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                -radians.sin(),
                0.0,
                radians.cos(),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        )
    }

    pub fn rotation_z(radians: f64) -> Self {
        Self::from_vec(
            4,
            4,
            vec![
                radians.cos(),
                -radians.sin(),
                0.0,
                0.0,
                radians.sin(),
                radians.cos(),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        )
    }

    pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Self {
        Self::from_vec(
            4,
            4,
            vec![
                1.0, x_y, x_z, 0.0, y_x, 1.0, y_z, 0.0, z_x, z_y, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        )
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn value_at(&self, row: usize, col: usize) -> f64 {
        self.data[self.index(row, col)]
    }

    pub fn set_value(&mut self, row: usize, col: usize, data: f64) {
        let index = self.index(row, col);
        self.data[index] = data;
    }

    pub fn transpose(&self) -> Self {
        let mut result = Matrix::new(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                result.set_value(col, row, self.value_at(row, col));
            }
        }

        result
    }

    pub fn determinant(&self) -> f64 {
        assert!(self.is_square());

        match self.rows {
            0 => 1.0,
            1 => self.value_at(0, 0),
            2 => {
                (self.value_at(0, 0) * self.value_at(1, 1))
                    - (self.value_at(0, 1) * self.value_at(1, 0))
            }
            _ => {
                let mut det = 0.0;

                for col in 0..self.cols {
                    det += self.value_at(0, col) * self.cofactor(0, col);
                }

                det
            }
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut result = Matrix::new(self.rows - 1, self.cols - 1);

        let mut result_row = 0;

        for curr_row in 0..self.rows {
            if curr_row == row {
                continue;
            }

            let mut result_col = 0;

            for curr_col in 0..self.cols {
                if curr_col == col {
                    continue;
                }

                result.set_value(result_row, result_col, self.value_at(curr_row, curr_col));

                result_col += 1;
            }

            result_row += 1;
        }
        result
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 1 {
            -self.minor(row, col)
        } else {
            self.minor(row, col)
        }
    }

    pub fn is_invertible(&self) -> bool {
        !math::equal(self.determinant(), 0.0)
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();

        if math::equal(det, 0.0) {
            return None;
        }

        let mut inverse = Matrix::new(self.rows, self.cols);

        for row in 0..self.rows {
            for col in 0..self.cols {
                let c = self.cofactor(row, col);
                inverse.set_value(col, row, c / det);
            }
        }
        Some(inverse)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows
            && self.cols == other.cols
            && self
                .data
                .iter()
                .zip(other.data.iter())
                .all(|(a, b)| math::equal(*a, *b))
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Matrix {
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;

                for k in 0..self.cols {
                    sum += self.value_at(i, k) * other.value_at(k, j);
                }

                result.set_value(i, j, sum);
            }
        }
        result
    }
}

impl Mul<Tuple> for &Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        let temp = Tuple::new(
            self.value_at(0, 0),
            self.value_at(0, 1),
            self.value_at(0, 2),
            self.value_at(0, 3),
        );
        let x = temp.dot(&rhs);

        let temp = Tuple::new(
            self.value_at(1, 0),
            self.value_at(1, 1),
            self.value_at(1, 2),
            self.value_at(1, 3),
        );
        let y = temp.dot(&rhs);

        let temp = Tuple::new(
            self.value_at(2, 0),
            self.value_at(2, 1),
            self.value_at(2, 2),
            self.value_at(2, 3),
        );
        let z = temp.dot(&rhs);

        let temp = Tuple::new(
            self.value_at(3, 0),
            self.value_at(3, 1),
            self.value_at(3, 2),
            self.value_at(3, 3),
        );
        let w = temp.dot(&rhs);

        Tuple::new(x, y, z, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Tuple;
    use std::f64::consts::PI;

    #[test]
    fn inspecting_4_x_4_matrix() {
        let m = Matrix::from_vec(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ],
        );

        assert_eq!(m.value_at(0, 0), 1.0);
        assert_eq!(m.value_at(0, 3), 4.0);
        assert_eq!(m.value_at(1, 0), 5.5);
        assert_eq!(m.value_at(1, 2), 7.5);
        assert_eq!(m.value_at(2, 2), 11.0);
        assert_eq!(m.value_at(3, 0), 13.5);
        assert_eq!(m.value_at(3, 2), 15.5);
    }

    #[test]
    fn inspecting_2_by_2_matrix() {
        let m = Matrix::from_vec(2, 2, vec![-3.0, 5.0, 1.0, -2.0]);

        assert_eq!(m.value_at(0, 0), -3.0);
        assert_eq!(m.value_at(0, 1), 5.0);
        assert_eq!(m.value_at(1, 0), 1.0);
        assert_eq!(m.value_at(1, 1), -2.0);
    }

    #[test]
    fn inspecting_3_by_3_matrix() {
        let m = Matrix::from_vec(3, 3, vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(m.value_at(0, 0), -3.0);
        assert_eq!(m.value_at(1, 1), -2.0);
        assert_eq!(m.value_at(2, 2), 1.0);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        let b = Matrix::from_vec(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        let b = Matrix::from_vec(
            4,
            4,
            vec![
                2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
            ],
        );

        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let b = Matrix::from_vec(
            4,
            4,
            vec![
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ],
        );

        let expected = Matrix::from_vec(
            4,
            4,
            vec![
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0,
            ],
        );

        assert_eq!(&a * &b, expected);
    }

    #[test]
    fn matrix_multiplied_by_tuple() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );

        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(&a * b, Tuple::new(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn identity_matrix() {
        let identity = Matrix::identity(4);

        assert_eq!(
            identity,
            Matrix::from_vec(
                4,
                4,
                vec![
                    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0
                ]
            )
        );
    }

    #[test]
    fn multiplying_matrix_by_identity() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
            ],
        );

        let identity = Matrix::identity(4);

        assert_eq!(&a * &identity, a);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ],
        );

        assert_eq!(
            a.transpose(),
            Matrix::from_vec(
                4,
                4,
                vec![
                    0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0
                ]
            )
        );
    }

    #[test]
    fn transposing_identity_matrix() {
        let identity = Matrix::identity(4);

        assert_eq!(identity.transpose(), Matrix::identity(4))
    }

    #[test]
    fn determinant_of_2_by_2_matrix() {
        let a = Matrix::from_vec(2, 2, vec![1.0, 5.0, -3.0, 2.0]);

        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn submatrix_of_3_by_3_matrix() {
        let a = Matrix::from_vec(3, 3, vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);

        assert_eq!(
            a.submatrix(0, 2),
            Matrix::from_vec(2, 2, vec![-3.0, 2.0, 0.0, 6.0])
        );
    }

    #[test]
    fn submatrix_of_4_by_4_matrix() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ],
        );

        assert_eq!(
            a.submatrix(2, 1),
            Matrix::from_vec(3, 3, vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0])
        );
    }

    #[test]
    fn minor_of_3_by_3_matrix() {
        let a = Matrix::from_vec(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        let b = a.submatrix(1, 0);

        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn calculate_cofactor_of_3_by_3_matrix() {
        let a = Matrix::from_vec(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculate_determinant_of_3_by_3_matrix() {
        let a = Matrix::from_vec(3, 3, vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);

        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn calculate_determinant_of_4_by_4_matrix() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
                -9.0,
            ],
        );

        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn testing_invertible_matrix_for_invertibility() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            ],
        );

        assert_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());
    }

    #[test]
    fn testing_noninvertible_matrix_for_invertibility() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ],
        );

        assert_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    #[test]
    fn calculating_inverse_of_matrix() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0,
                4.0,
            ],
        );

        let b = a.inverse().unwrap();

        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b.value_at(3, 2), -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b.value_at(2, 3), 105.0 / 532.0);
        assert_eq!(
            b,
            Matrix::from_vec(
                4,
                4,
                vec![
                    0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
                    -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639
                ]
            )
        );
    }

    #[test]
    fn calculate_another_inverse() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
            ],
        );

        assert_eq!(
            a.inverse().unwrap(),
            Matrix::from_vec(
                4,
                4,
                vec![
                    -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077,
                    0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308
                ]
            )
        );
    }

    #[test]
    fn multiplying_product_by_inverse() {
        let a = Matrix::from_vec(
            4,
            4,
            vec![
                3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0,
                1.0,
            ],
        );
        let b = Matrix::from_vec(
            4,
            4,
            vec![
                8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
            ],
        );

        let c = &a * &b;

        assert_eq!(&c * &b.inverse().unwrap(), a);
    }

    #[test]
    fn multiplying_by_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(&transform * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_inverse_of_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(&inv * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(&transform * v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(&transform * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(&transform * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_inverse_of_scaling_matrix() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(&inv * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative_number() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(&transform * p, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let full_quarter = Matrix::rotation_x(PI / 2.0);

        assert_eq!(
            &half_quarter * p,
            Tuple::point(0.0, (2.0f64).sqrt() / 2.0, (2.0f64).sqrt() / 2.0)
        );

        assert_eq!(&full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();

        assert_eq!(
            &inv * p,
            Tuple::point(0.0, (2.0f64).sqrt() / 2.0, -(2.0f64).sqrt() / 2.0)
        );
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(PI / 4.0);
        let full_quarter = Matrix::rotation_y(PI / 2.0);

        assert_eq!(
            &half_quarter * p,
            Tuple::point((2.0f64).sqrt() / 2.0, 0.0, (2.0f64).sqrt() / 2.0)
        );
        assert_eq!(&full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(PI / 4.0);
        let full_quarter = Matrix::rotation_z(PI / 2.0);

        assert_eq!(
            &half_quarter * p,
            Tuple::point(-(2.0f64).sqrt() / 2.0, (2.0f64).sqrt() / 2.0, 0.0)
        );
        assert_eq!(&full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(&transform * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(&transform * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(&transform * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(&transform * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(&transform * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(&transform * p, Tuple::point(2.0, 3.0, 7.0));
    }
}
