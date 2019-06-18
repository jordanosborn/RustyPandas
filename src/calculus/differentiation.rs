#![allow(dead_code)]

use crate::linalg::Vector;
use crate::Function;
pub fn diff(f: fn(&Vec<f64>) -> f64, position: Vec<f64>, step: f64) -> Vec<f64> {
    let mut ret = Vec::with_capacity(position.len());
    for i in 0..position.len() {
        let mut plus = position.to_owned();
        plus[i] += step;
        let mut minus = position.to_owned();
        minus[i] -= step;
        ret.push((f(&plus) - f(&minus)) / (2.0 * step));
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_diff() {
        let f: fn(&Vec<f64>) -> f64 = |v| v[0] * v[0] + v[1] * v[1];
        let a = diff(f, vec![2.0, 2.0], 0.01);
        print!("{:?}", a);
        assert!((a[0] - 4.0).abs() < 0.0001);
    }
}