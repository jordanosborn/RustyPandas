#![allow(dead_code)]
use num::Num;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Dual<T: Num> {
    pub a: T,
    pub b: T
}


impl<T: Num> Dual<T> {
    fn new(a: T, b: T) -> Self {
        Self {
            a, b
        }
    }
}

impl<T: Num> std::ops::Add for Dual<T> {
    type Output = Dual<T>;

    fn add(self, other: Dual<T>) -> Self::Output {
        Self::Output {
            a: self.a + other.a,
            b: self.b + other.b
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn add() {
        let d1 = Dual::new(34.0, 1.0);
        let d2 = Dual::new(23.0, 0.0);
        assert_eq![d1 + d2, Dual::new(57.0, 1.0)];
    }
}