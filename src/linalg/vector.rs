use rayon::prelude::*;
#[derive(Clone)]
pub struct Vector {
    pub data: Vec<f64>,
}

impl std::ops::Index<usize> for Vector {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Vector {
    pub fn len(self) -> usize {
        self.data.len()
    }
}


impl std::ops::Mul<Vector> for Vector {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(a, b)| a * b)
            .sum()
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }
}
impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }
}

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            data: rhs.data.par_iter().map(|x| self * x).collect(),
        }
    }
}

impl std::ops::BitXor<Vector> for Vector {
    type Output = Vector;
    fn bitxor(self, rhs: Vector) -> Self::Output {
        assert!(rhs.data.len() == 3 && self.data.len() == 3);
        Vector {
            data: vec![
                self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
                self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
                self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
            ],
        }
    }
}