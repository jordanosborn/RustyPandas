
#[inline]
pub fn sinc<T: num::Float>(x: T) -> T {
    if x.is_zero() {
        T::one()
    } else {
        x.sin() / x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sinc() {
        assert!(f64::abs(sinc(0.0) - 1.0) < 1e-10_f64);
    }
}