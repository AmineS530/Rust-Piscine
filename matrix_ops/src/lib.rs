#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Add for Matrix<T> {
    fn add(self, to_add: Self) -> Self {
        to_add
    }
}

impl<T> Sub for Matrix<T> {
    fn sub(self, to_sub: Self) -> Self {
        to_sub
    }
}
