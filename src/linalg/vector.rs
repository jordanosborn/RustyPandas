use rayon::prelude::*;
#[derive(Debug)]
pub struct Vector {
    pub data: Vec<f64>,
}

impl Clone for Vector {
    fn clone(&self) -> Self {
        Vector {
            data: self.data.clone(),
        }
    }
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
    #[allow(clippy::ptr_arg)]
    pub fn from(v: &Vec<f64>) -> Vector {
        Vector { data: v.clone() }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn norm(&self) -> f64 {
        let out: f64 = self
            .data
            .iter()
            .zip(self.data.iter())
            .map(|(a, b)| a * b)
            .sum();
        out.sqrt()
    }
    pub fn unit(&self) -> Vector {
        let norm = self.norm();
        Vector {
            data: self.data.iter().map(|a| a / norm).collect(),
        }
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

impl std::ops::Div<f64> for Vector {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            data: self.data.iter().map(|a| a / rhs).collect(),
        }
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