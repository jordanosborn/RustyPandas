#![allow(dead_code)]
use num::{Num, Float};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Dual<T: Num + Copy> {
    pub a: T,
    pub b: T,
}

impl<T: Num + Copy> Dual<T> {
    fn new(a: T, b: T) -> Self {
        Self { a, b }
    }
}

impl<T: Float + Copy> Dual<T> {
    fn sin(self) -> Self {
        Self {
            a: self.a.sin(), b: self.b.cos()
        }
    }

    fn cos(self) -> Self {
        Self {
            a: self.a.cos(), b: -self.b.sin()
        }
    }

    fn tan(self) -> Self {
        Self {
            a: self.a.tan(),
            b: self.b.cos().powf(T::from(-2.0_f64).unwrap())
        }
    }
}

impl<T: Num + Copy> std::ops::Add for Dual<T> {
    type Output = Dual<T>;

    fn add(self, other: Dual<T>) -> Self::Output {
        Self::Output {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl<T: Num + Copy> std::ops::Sub for Dual<T> {
    type Output = Dual<T>;

    fn sub(self, other: Dual<T>) -> Self::Output {
        Self::Output {
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}

impl<T: Num + Copy> std::ops::Mul for Dual<T> {
    type Output = Dual<T>;

    fn mul(self, other: Dual<T>) -> Self::Output {
        Self::Output {
            a: self.a * other.a,
            b: self.a * other.b + self.b * other.a,
        }
    }
}

impl<T: Num + Copy + std::ops::Neg<Output = T>> std::ops::Neg for Dual<T> {
    type Output = Dual<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            a: -self.a,
            b: -self.b,
        }
    }
}

impl<T: Num + Copy, V: Num + Copy> std::ops::Div<V> for Dual<T>
where
    T: std::ops::Div<V, Output = T>,
{
    type Output = Dual<T>;

    fn div(self, other: V) -> Self::Output {
        Self::Output {
            a: self.a / other,
            b: self.b / other,
        }
    }
}

impl<T: Num + Copy + std::ops::Neg<Output = T>> std::ops::Div for Dual<T> {
    type Output = Dual<T>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: Dual<T>) -> Self::Output {
        let divisor = self.a * other.a;
        Self::Output {
            a: (self.a * other.a) / divisor,
            b: (other.a * self.b - other.b * self.a) / divisor,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let d1 = Dual::new(34.0, 1.0);
        let d2 = Dual::new(23.0, 0.0);
        assert_eq![d1 + d2, Dual::new(57.0, 1.0)];
    }

    #[test]
    fn mul() {
        let d1 = Dual::new(4.0, 1.0);
        let d2 = Dual::new(5.0, 3.0);
        assert_eq![d1 * d2, Dual::new(20.0, 17.0)]
    }
}
