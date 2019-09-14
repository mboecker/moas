#[derive(Debug, Clone)]
pub struct Matrix<T> {
    d: Vec<T>,
    size: usize,
}

impl<T> Matrix<T>
where
    T: Default,
{
    pub fn new(a: usize) -> Matrix<T> {
        Matrix {
            d: (0..a * a).map(|_| T::default()).collect(),
            size: a,
        }
    }
}

impl<T> Matrix<T> {
    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.d[i + j * self.size]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.d[i + j * self.size]
    }
}
