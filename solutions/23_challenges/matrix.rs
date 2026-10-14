// matrix.rs (challenge) - solution
//
// `Index` returns a reference into the data, so `m[(r, c)]` reads like a
// plain value and `IndexMut` makes assignment work too. The explicit
// bounds check matters: (0, 2) in a 2x2 matrix is flat index 2, which
// exists but belongs to the next row.
//
// `impl Mul for &Matrix` borrows both operands, so `&a * &b` leaves
// `a` and `b` usable afterwards.

use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Clone, PartialEq)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix { rows, cols, data: vec![0.0; rows * cols] }
    }

    fn identity(n: usize) -> Self {
        let mut m = Matrix::new(n, n);
        for i in 0..n {
            m[(i, i)] = 1.0;
        }
        m
    }

    fn from_rows(rows: Vec<Vec<f64>>) -> Self {
        let cols = rows.first().map_or(0, Vec::len);
        assert!(rows.iter().all(|row| row.len() == cols), "rows must all have the same length");
        Matrix { rows: rows.len(), cols, data: rows.into_iter().flatten().collect() }
    }

    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn transpose(&self) -> Matrix {
        let mut t = Matrix::new(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                t[(c, r)] = self[(r, c)];
            }
        }
        t
    }

    fn checked_mul(&self, other: &Matrix) -> Option<Matrix> {
        if self.cols != other.rows {
            return None;
        }
        let mut out = Matrix::new(self.rows, other.cols);
        for r in 0..self.rows {
            for c in 0..other.cols {
                out[(r, c)] = (0..self.cols).map(|k| self[(r, k)] * other[(k, c)]).sum();
            }
        }
        Some(out)
    }

    fn offset(&self, (r, c): (usize, usize)) -> usize {
        assert!(
            r < self.rows && c < self.cols,
            "index ({r}, {c}) out of bounds for {}x{} matrix",
            self.rows,
            self.cols
        );
        r * self.cols + c
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &f64 {
        &self.data[self.offset(index)]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f64 {
        let i = self.offset(index);
        &mut self.data[i]
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        self.checked_mul(other).unwrap_or_else(|| {
            panic!(
                "dimension mismatch: {}x{} * {}x{}",
                self.rows, self.cols, other.rows, other.cols
            )
        })
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros() {
        let m = Matrix::new(2, 3);
        assert_eq!((m.rows(), m.cols()), (2, 3));
        assert_eq!(m[(1, 2)], 0.0);
    }

    #[test]
    fn index_and_index_mut() {
        let mut m = Matrix::new(2, 2);
        m[(0, 1)] = 5.0;
        m[(1, 0)] += 2.0;
        assert_eq!(m, Matrix::from_rows(vec![vec![0.0, 5.0], vec![2.0, 0.0]]));
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let m = Matrix::new(2, 2);
        let _ = m[(0, 2)];
    }

    #[test]
    #[should_panic]
    fn ragged_rows() {
        Matrix::from_rows(vec![vec![1.0, 2.0], vec![3.0]]);
    }

    #[test]
    fn identity() {
        let i = Matrix::identity(3);
        for r in 0..3 {
            for c in 0..3 {
                assert_eq!(i[(r, c)], if r == c { 1.0 } else { 0.0 });
            }
        }
    }

    #[test]
    fn transpose() {
        let m = Matrix::from_rows(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        let t = Matrix::from_rows(vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]);
        assert_eq!(m.transpose(), t);
    }

    #[test]
    fn multiply() {
        let a = Matrix::from_rows(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);
        let b = Matrix::from_rows(vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]);
        let expected = Matrix::from_rows(vec![vec![58.0, 64.0], vec![139.0, 154.0]]);
        assert_eq!(a.checked_mul(&b), Some(expected.clone()));
        assert_eq!(&a * &b, expected);
    }

    #[test]
    fn identity_is_neutral() {
        let a = Matrix::from_rows(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(&a * &Matrix::identity(2), a);
        assert_eq!(&Matrix::identity(2) * &a, a);
    }

    #[test]
    fn mismatched_dimensions() {
        let a = Matrix::new(2, 3);
        assert_eq!(a.checked_mul(&Matrix::new(2, 3)), None);
    }

    #[test]
    #[should_panic(expected = "dimension mismatch")]
    fn mismatched_operator_panics() {
        let _ = &Matrix::new(2, 3) * &Matrix::new(2, 3);
    }
}
