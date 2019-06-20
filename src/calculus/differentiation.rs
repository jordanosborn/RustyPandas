#![allow(dead_code)]
use crate::linalg::vector::Vector;
#[allow(unused_imports, unused_attributes)]
#[macro_use]
use crate::{create_equality_test, create_inequality_test, create_panic_test, create_approx_test};

pub fn diff(f: &dyn Fn(&Vec<f64>) -> f64, position: &[f64], step: f64) -> Vector {
    let mut ret = Vec::with_capacity(position.len());
    for i in 0..position.len() {
        let mut plus = position.to_owned();
        plus[i] += step;
        let mut minus = position.to_owned();
        minus[i] -= step;
        ret.push((f(&plus) - f(&minus)) / (2.0 * step));
    }
    Vector { data: ret }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn diff_test(f: &dyn Fn(&Vec<f64>) -> f64, position: &[f64], var: usize) -> f64 {
        let a = diff(&f, position, 0.01);
        print!("{:?}", a);
        assert!(a.len() > var);
        a[var]
    }

    create_approx_test!(quad_test, diff_test,
        &|var: &Vec<f64>| var[0]* var[0] + 7.0, &[2.0], 0 => 4.0, 0.0001
    );

    #[test]
    fn test_diff() {
        let f: fn(&Vec<f64>) -> f64 = |v| v[0] * v[0] + v[1] * v[1];
        let a = diff(&f, &[2.0, 2.0], 0.01);
        print!("{:?}", a);
        assert!((a[0] - 4.0).abs() < 0.0001);
    }
    #[test]
    fn test_diff2() {
        let f: fn(&Vec<f64>) -> f64 = |v| v[0].powf(4.0) + v[1] * v[1];
        let a = diff(&f, &[2.0, 2.0], 0.01);
        print!("{:?}", a);
        assert!((a[0] - 32.0).abs() < 0.001);
    }
}