use super::{least_squares, weighted_least_squares, Function};
use crate::calculus::differentiation::diff;
use crate::linalg;


pub fn jacobian<const M: usize, const N: usize>(f: Function, x: &[f64; M], args: &[f64; N]) {}
