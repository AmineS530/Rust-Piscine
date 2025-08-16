#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
pub trait Scalar {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item {
        0 as u32
    }
    fn one() -> Self::Item {
        1 as u32
    }
}

impl Scalar for u64 {
    type Item = u64;
    fn zero() -> Self::Item {
        0 as u64
    }
    fn one() -> Self::Item {
        1 as u64
    }
}

impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self::Item {
        0 as i32
    }
    fn one() -> Self::Item {
        1 as i32
    }
}

impl Scalar for i64 {
    type Item = i64;
    fn zero() -> Self::Item {
        0 as i64
    }
    fn one() -> Self::Item {
        1 as i64
    }
}

impl Scalar for f32 {
    type Item = f32;
    fn zero() -> Self::Item {
        0.0
    }
    fn one() -> Self::Item {
        1.0
    }
}

impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self::Item {
        0.0
    }
    fn one() -> Self::Item {
        1.0
    }
}

impl<T: Scalar<Item = T> + std::clone::Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut m = Matrix::zero(n, n);
        for i in 0..n {
            m.0[i][i] = T::one();
        }
        m
    }
}
