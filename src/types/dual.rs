#![allow(dead_code)]
use num::{Float, Num};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Dual<T: Num + Copy> {
    pub a: T,
    pub b: T,
}

impl<T: Num + Copy + std::ops::Neg> std::fmt::Display for Dual<T>
where
    T: std::fmt::Display + PartialOrd,
    <T as std::ops::Neg>::Output: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}e",
            self.a,
            if self.b < T::zero() {
                format!["- {}", -self.b]
            } else {
                format!["+ {}", self.b]
            }
        )
    }
}

impl<T: Num + Copy> Dual<T> {
    fn new(a: T, b: T) -> Self {
        Self { a, b }
    }
}

impl<T: Num + Copy> From<T> for Dual<T> {
    fn from(value: T) -> Self {
        Self {
            a: value,
            b: T::zero(),
        }
    }
}

impl<T: Float + Copy> Dual<T> {
    #[inline]
    fn exp(self) -> Self {
        Self {
            a: self.a.exp(),
            b: self.b * self.a.exp(),
        }
    }

    #[inline]
    fn powf(self, other: T) -> Self {
        Self {
            a: self.a.powf(other),
            b: self.b * other * self.a.powf(other - T::one()),
        }
    }

    // #[inline]
    // fn pow(self, other: Dual<T>) -> Self {
    //     Self {
    //         a:
    //     }
    // }

    #[inline]
    fn sin(self) -> Self {
        Self {
            a: self.a.sin(),
            b: self.b * self.a.cos(),
        }
    }

    #[inline]
    fn sinh(self) -> Self {
        Self {
            a: self.a.sinh(),
            b: self.b * self.a.cosh(),
        }
    }

    #[inline]
    fn cos(self) -> Self {
        Self {
            a: self.a.cos(),
            b: -self.b * self.a.sin(),
        }
    }

    #[inline]
    fn cosh(self) -> Self {
        Self {
            a: self.a.cosh(),
            b: self.b * self.a.sinh(),
        }
    }

    #[inline]
    fn tan(self) -> Self {
        Self {
            a: self.a.tan(),
            b: self.b * self.a.cos().powf(T::from(-2.0_f64).unwrap()),
        }
    }

    #[inline]
    fn tanh(self) -> Self {
        Self {
            a: self.a.tanh(),
            b: self.b * (T::one() - self.a.tanh().powf(T::from(2.0_f64).unwrap())),
        }
    }
}

impl<T: Num + Copy> std::ops::Add for Dual<T> {
    type Output = Dual<T>;

    #[inline]
    fn add(self, other: Dual<T>) -> Self::Output {
        Self::Output {
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}

impl<T: Num + Copy> std::ops::Sub for Dual<T> {
    type Output = Dual<T>;

    #[inline]
    fn sub(self, other: Dual<T>) -> Self::Output {
        Self::Output {
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}

impl<T: Num + Copy> std::ops::Mul for Dual<T> {
    type Output = Dual<T>;

    #[inline]
    fn mul(self, other: Dual<T>) -> Self::Output {
        Self::Output {
            a: self.a * other.a,
            b: self.a * other.b + self.b * other.a,
        }
    }
}

impl<T: Num + Copy + std::ops::Neg<Output = T>> std::ops::Neg for Dual<T> {
    type Output = Dual<T>;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            a: -self.a,
            b: -self.b,
        }
    }
}

impl<T: Num + Copy, V: Num + Copy> std::ops::Mul<V> for Dual<T>
where
    T: std::ops::Mul<V, Output = T>,
{
    type Output = Dual<T>;
    #[inline]
    fn mul(self, other: V) -> Self::Output {
        Self::Output {
            a: self.a * other,
            b: self.b * other,
        }
    }
}

/*
    Symmetric lhs and rhs multiplication only implemented for f64...
*/
impl<T: Num + Copy> std::ops::Mul<Dual<T>> for f64
where
    T: std::ops::Mul<f64, Output = T>,
{
    type Output = Dual<T>;
    #[inline]
    fn mul(self, other: Dual<T>) -> Self::Output {
        Self::Output {
            a: other.a * self,
            b: other.b * self,
        }
    }
}

impl<T: Num + Copy, V: Num + Copy> std::ops::Div<V> for Dual<T>
where
    T: std::ops::Div<V, Output = T>,
{
    type Output = Dual<T>;
    #[inline]
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
    #[inline]
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

    #[test]
    fn vector_func() {
        fn cos_x_y(x: Dual<f64>, y: Dual<f64>) -> Dual<f64> {
            (x * y).cos()
        }

        fn dcos_x_y_dy(x: f64, y: f64) -> f64 {
            -x * (x * y).sin()
        }

        let d1 = Dual::new(5.0, 0.0);
        let d2 = Dual::new(2.0, 1.0);
        let d3 = cos_x_y(d1, d2);
        assert![(d3.b - dcos_x_y_dy(d1.a, d2.a)).abs() < 1e-5_f64]
    }

    #[test]
    fn sub() {
        let d1 = Dual::new(4.0, 1.0);
        let d2 = Dual::new(5.0, 3.0);
        assert_eq![d2 - d1, Dual::new(1.0, 2.0)]
    }

    #[test]
    fn display() {
        println!["{}", Dual::new(67.0, -89.0)]
    }
}
