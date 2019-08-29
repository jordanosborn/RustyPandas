use super::{least_squares, weighted_least_squares, Function};
use crate::calculus::differentiation::diff;
use crate::linalg;

pub fn jacobian(f: Function, x: &[f64], args: &[f64]) -> f64 {
    f(x, args)
}
