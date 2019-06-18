use rayon::prelude::*;
pub impl std::ops::Mul<Vec<f64>> for Vec<f64> {
    type Output = f64;
    fn mul(self, rhs: Self) -> self::Output {
        self.par_iter().zip(rhs.par_iter()).map(|(a, b)| a * b).sum()
    }
}

pub impl std::ops::BitXor<Vec<f64>> for Vec<f64> {
    type Output = Vec<f64>;
    fn bitxor(self, rhs: Vec<f64>) -> self::Output {
        assert(rhs.len() == 3 && self.len() == 3);
        vec![self[1]*rhs[2] - self[2]*rhs[1], self[2]*rhs[0] - self[0]*rhs[2], self[0]*rhs[1] - self[1]*rhs[0]]
    }
}