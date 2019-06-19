#![allow(dead_code)]

use crate::calculus::differentiation::*;
use rayon::prelude::*;
type Function<'a> = &'a dyn Fn(&Vec<f64>, &Vec<f64>) -> f64;
type Cost<'a> = &'a dyn Fn(&dyn Fn(&Vec<f64>, &Vec<f64>) -> f64, Vec<f64>, Vec<Vec<f64>>, Vec<f64>) -> f64;
type CostWeighted<'a> = &'a Fn(&dyn Fn(&Vec<f64>, &Vec<f64>) -> f64, Vec<f64>, Vec<Vec<f64>>, Vec<f64>, Vec<f64>) -> f64;

pub fn least_squares(f: Function, params: Vec<f64>, vars: Vec<Vec<f64>>, y: Vec<f64>) -> f64 {
    vars.iter()
        .zip(y.iter())
        .map(|(x, y)| (y - f(x, &params)).powf(2.0))
        .sum()
}
pub fn weighted_least_squares(
    f: Function,
    params: Vec<f64>,
    vars: Vec<Vec<f64>>,
    y: Vec<f64>,
    weights: Vec<f64>,
) -> f64 {
    vars.iter()
        .zip(y.iter())
        .zip(weights.iter())
        .map(|((x, y), w)| w * (y - f(x, &params).powf(2.0)))
        .sum()
}
pub fn fit_weighted<'a>(
    f: Function<'a>,
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
        &weighted_least_squares
    };
    let cost_func: Cost = &(|f, params, vars, y| cost_function(f, params, vars, y, weights.to_owned()));
    fit(f, vars, y, bounds, Some(&cost_func), threshold)
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
        &least_squares
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