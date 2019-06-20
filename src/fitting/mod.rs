#![allow(dead_code)]
#[allow(unused_imports)]
use itertools::Itertools;

use crate::calculus::differentiation::*;
use crate::linalg::vector::Vector;
type Function<'a> = &'a dyn Fn(&[f64], &[f64]) -> f64;
type Cost<'a> = &'a dyn Fn(&dyn Fn(&[f64], &[f64]) -> f64, &[f64], &[Vec<f64>], &[f64]) -> f64;
type CostWeighted<'a> =
    &'a Fn(&dyn Fn(&[f64], &[f64]) -> f64, &[f64], &[Vec<f64>], &[f64], &[f64]) -> f64;

pub fn least_squares(f: Function, params: &[f64], vars: &[Vec<f64>], y: &[f64]) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    vars.iter()
        .zip(y.iter())
        .map(|(x, y)| (y - f(x, &params)).powf(2.0))
        .map(|x| x / vars.len() as f64)
        .sum()
}
pub fn weighted_least_squares(
    f: Function,
    params: &[f64],
    vars: &[Vec<f64>],
    y: &[f64],
    weights: &[f64],
) -> f64 {
    vars.iter()
        .zip(y.iter())
        .zip(weights.iter())
        .map(|((x, y), w)| w * (y - f(x, &params).powf(2.0)))
        .sum()
}

#[allow(clippy::too_many_arguments)]
pub fn fit_weighted(
    f: Function,
    vars: &[Vec<f64>],
    y: &[f64],
    weights: &[f64],
    bounds: (Vec<f64>, Vec<f64>),
    cost_function: Option<CostWeighted>,
    threshold: Option<f64>,
    learning_rates: (Option<f64>, Option<f64>),
    iterations: Option<usize>,
) -> (Vec<f64>, f64) {
    let cost_function = cost_function.unwrap_or(&weighted_least_squares);
    let cost_func: Cost = &(|f, params, vars, y| cost_function(f, params, vars, y, weights));
    fit(
        f,
        vars,
        y,
        bounds,
        Some(&cost_func),
        threshold,
        learning_rates,
        iterations,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn fit(
    f: Function,
    vars: &[Vec<f64>],
    y: &[f64],
    bounds: (Vec<f64>, Vec<f64>),
    cost_function: Option<Cost>,
    threshold: Option<f64>,
    learning_rates: (Option<f64>, Option<f64>),
    iterations: Option<usize>,
) -> (Vec<f64>, f64) {
    let cost_function = cost_function.unwrap_or(&least_squares);
    let threshold = threshold.unwrap_or(0.5);
    let eta = learning_rates.0.unwrap_or(0.1);
    let gamma = learning_rates.1.unwrap_or(0.9);
    let iterations = iterations.unwrap_or(100_000);
    let cost_function_params = |params: &Vec<f64>| cost_function(f, params, vars, y);
    //Use diff to minimise cost
    let mut params = bounds.0;

    let mut _found = false;
    let mut delta = Vector {
        data: vec![0.0; params.len()],
    };
    for _ in 0..iterations {
        let gradient = diff(
            &cost_function_params,
            &(Vector::from(&params) - gamma * delta.clone()).data,
            0.001,
        )
        .unit();
        let cost = cost_function_params(&params);
        if cost < threshold {
            _found = true;
            break;
        }
        delta = gamma * delta + eta * gradient.clone();
        params = (Vector::from(&params) - delta.clone()).data;
    }
    let cost = cost_function_params(&params);
    (params, cost)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_quadratic_fit() {
        let vars: Vec<Vec<f64>> = (0..20).map(f64::from).map(|x| vec![x]).collect();
        let y: Vec<f64> = (0..20)
            .map(f64::from)
            .map(|x| 5.0 * x.powf(2.0) + 2.0 * x + -8.0)
            .collect();
        let bounds = (vec![-10.0, -10.0, -10.0], vec![100.0, 100.0, 100.0]);
        let f = &(|vars: &[f64], params: &[f64]| {
            params[0] * vars[0].powf(2.0) + params[1] * vars[0] + params[2]
        });
        println!(
            "{:?}",
            fit(f, &vars, &y, bounds, None, None, (None, None), None)
        );
    }
    #[test]
    fn test_2d_fit() {
        let vars: Vec<Vec<f64>> = (0..20)
            .cartesian_product(0..20)
            .map(|(x, y)| vec![f64::from(x), f64::from(y)])
            .collect();
        let y: Vec<f64> = (0..20)
            .cartesian_product(0..20)
            .map(|(x, y)| (f64::from(x), f64::from(y)))
            .map(|(x, y)| 2.0 * x.powf(2.0) + 3.0 * y.powf(2.0))
            .collect();
        let bounds = (vec![-10.0, -10.0], vec![50.0, 50.0]);
        let f = &(|vars: &[f64], params: &[f64]| {
            params[0] * vars[0].powf(2.0) + params[1] * vars[1].powf(2.0)
        });
        println!(
            "{:?}",
            fit(f, &vars, &y, bounds, None, None, (None, None), None)
        );
    }
}