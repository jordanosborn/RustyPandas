
use num::Zero;
use rayon::prelude::*;
#[derive(Debug)]
pub struct Vector {
    pub data: Vec<f64>,
}

impl Clone for Vector {
    fn clone(&self) -> Self {
        Self {
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
    pub fn from(v: &[f64]) -> Self {
        Self { data: v.to_vec() }
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
    pub fn unit(&self) -> Self {
        let norm = self.norm();
        if f64::is_zero(&norm) {
            self.to_owned()
        } else {
            Self {
                data: self.data.iter().map(|a| a / norm).collect(),
            }
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
        Self {
            data: self.data.iter().map(|a| a / rhs).collect(),
        }
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
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
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
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
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        assert!(rhs.data.len() == 3 && self.data.len() == 3);
        Self {
            data: vec![
                self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
                self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
                self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0],
            ],
        }
    }
}