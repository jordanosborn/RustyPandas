#![allow(dead_code)]

use crate::calculus::differentiation::*;
use crate::Function;
use rayon::prelude::*;
type Cost = fn(Function, Vec<f64>, Vec<Vec<f64>>, Vec<f64>) -> f64;
type CostWeighted = fn(Function, Vec<f64>, Vec<Vec<f64>>, Vec<f64>, Vec<f64>) -> f64;

pub fn least_squares(f: Function, params: Vec<f64>, vars: Vec<Vec<f64>>, y: Vec<f64>) -> f64 {
    vars.par_iter()
        .zip(y.par_iter())
        .map(|(x, y)| (y - f(x, &params).powf(2.0)))
        .sum()
}
pub fn weighted_least_squares(
    f: Function,
    params: Vec<f64>,
    vars: Vec<Vec<f64>>,
    y: Vec<f64>,
    weights: Vec<f64>,
) -> f64 {
    vars.par_iter()
        .zip(y.par_iter())
        .zip(weights.par_iter())
        .map(|((x, y), w)| w * (y - f(x, &params).powf(2.0)))
        .sum()
}
pub fn fit_weighted(
    f: Function,
    vars: Vec<Vec<f64>>,
    y: Vec<f64>,
    weights: Vec<f64>,
    bounds: Vec<(f64, f64)>,
    cost_function: Option<CostWeighted>,
    threshold: Option<f64>,
) {
    let cost_function = if let Some(c) = cost_function {
        c
    } else {
        weighted_least_squares
    };
    let cost_function = |(f, params, vars, y)| cost_function(f, params, vars, y, weights);
    //Use diff to minimise cost
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
    //Use diff to minimise cost
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_linear() {}
}