// matrix.rs (challenge)
//
// Implement a 2D matrix of f64 values that feels built in. Indexing should
// look like `m[(row, col)]` and multiplication like `&a * &b`.
//
// The data lives in a single flat Vec in "row-major" order: row 0's values,
// then row 1's, and so on. Element (r, c) is at index `r * cols + c`.
//
// Requirements:
//   1. Finish the constructors and methods below.
//      `from_rows` must panic if the rows have different lengths.
//   2. Implement `Index<(usize, usize)>` and `IndexMut<(usize, usize)>`.
//      Out-of-range indices must panic (even when `r * cols + c` would
//      still land inside the Vec!).
//   3. `checked_mul` returns None when the shapes don't fit: an (n x m)
//      matrix can only be multiplied by an (m x p) one.
//   4. Implement `Mul` for `&Matrix` (references, so multiplying doesn't
//      consume the inputs). It should panic with a message containing
//      "dimension mismatch" when `checked_mul` would return None.
//
// Further reading:
//   - Matrix multiplication: https://en.wikipedia.org/wiki/Matrix_multiplication
//   - Operator overloading: https://doc.rust-lang.org/rust-by-example/trait/ops.html
//   - Index: https://doc.rust-lang.org/std/ops/trait.Index.html
//   - IndexMut: https://doc.rust-lang.org/std/ops/trait.IndexMut.html

// I AM NOT DONE

use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Clone, PartialEq)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    /// A rows x cols matrix full of zeros.
    fn new(rows: usize, cols: usize) -> Self {
        todo!()
    }

    /// An n x n matrix with ones on the diagonal and zeros elsewhere.
    fn identity(n: usize) -> Self {
        todo!()
    }

    fn from_rows(rows: Vec<Vec<f64>>) -> Self {
        todo!()
    }

    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn transpose(&self) -> Matrix {
        todo!()
    }

    fn checked_mul(&self, other: &Matrix) -> Option<Matrix> {
        todo!()
    }
}

// TODO: impl Index<(usize, usize)> for Matrix { ... }

// TODO: impl IndexMut<(usize, usize)> for Matrix { ... }

// TODO: impl Mul for &Matrix { ... }

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
