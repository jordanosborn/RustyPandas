#![allow(dead_code)]

use crate::calculus::*;
use crate::Function;
use rayon::prelude::*;
type Cost = fn(Function, Vec<f64>, Vec<Vec<f64>>, Vec<f64>) -> f64;

pub fn least_squares(
    f: Function,
    params: Vec<f64>,
    vars: Vec<Vec<f64>>,
    y: Vec<f64>,
) -> f64 {
    vars.par_iter()
        .zip(y.par_iter())
        .map(|(x, y)| (y - f(x, &params).powf(2.0)))
        .sum()
}

pub fn fit(
    f: Function,
    vars: Vec<Vec<f64>>,
    y: Vec<f64>,
    bounds: Vec<(f64, f64)>,
    cost_function: Option<Cost>,
    threshold: Option<f64>,
) {
    let cost_function = if let Some(c) = cost_function {
        c
    } else {
        least_squares
    };

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_linear() {}
}