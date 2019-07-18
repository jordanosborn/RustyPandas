#[derive(Default, Clone, Debug)]
pub struct Matrix<T, const M: usize, const N: usize> {
    data: Vec<Vec<T>>,
    pub rows: usize,
    pub columns: usize,
}

impl<T, const M: usize, const N: usize> Matrix<T, { M }, { N }> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            rows: M,
            columns: N,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let x = Matrix::new();
        println!("{}", x.clone().dot(x));
    }
}
