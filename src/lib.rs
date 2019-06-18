#[allow(unused_imports)]
use rayon::prelude::*;

mod calculus;
mod fitting;
mod linalg;

type Function = fn(&Vec<f64>, &Vec<f64>) -> f64;
#[allow(unused_macros)]
macro_rules! new_matrix {
    ($x: ident, $m: expr, $n: expr) => {
        #[derive(Debug, Clone)]
        struct $x<T> {
            pub data: Vec<T>,
            rows: usize,
            cols: usize,
        }
        #[allow(dead_code)]
        impl<T: Clone + std::ops::Mul> $x<T> {
            fn new(data: [[T; $n]; $m]) -> $x<T> {
                let mut v: Vec<T> = Vec::with_capacity($m * $n);
                data.iter().for_each(|d| v.extend(d.to_vec()));
                $x {
                    data: v,
                    rows: $m,
                    cols: $n,
                }
            }
            fn dot(self, rhs: $x<T>) -> $x<<T as std::ops::Mul>::Output> {
                let mut out = Vec::with_capacity(self.data.len());
                for (x, y) in rhs.data.into_iter().zip(self.data.into_iter()) {
                    out.push(x * y);
                }
                $x {
                    data: out,
                    rows: $m,
                    cols: $n,
                }
            }
        }
        impl<T: std::ops::Add + Clone + Clone + Copy> std::ops::Add<$x<T>> for $x<T> {
            type Output = $x<T::Output>;
            fn add(self, rhs: $x<T>) -> $x<T::Output> {
                let mut out = Vec::with_capacity(self.data.len());
                for (x, y) in rhs.data.into_iter().zip(self.data.into_iter()) {
                    out.push(x + y);
                }
                $x {
                    data: out,
                    rows: $m,
                    cols: $n,
                }
            }
        }
        impl<T: std::ops::Mul + Clone + Copy> std::ops::Mul<T> for $x<T> {
            type Output = $x<T::Output>;
            fn mul(self, rhs: T) -> $x<T::Output> {
                let mut out = Vec::with_capacity(self.data.len());
                for x in self.data.into_iter() {
                    out.push(rhs * x);
                }
                $x {
                    data: out,
                    rows: $m,
                    cols: $n,
                }
            }
        }
        impl<T: std::ops::Sub + Clone + Clone + Copy> std::ops::Sub<$x<T>> for $x<T> {
            type Output = $x<T::Output>;
            fn sub(self, rhs: $x<T>) -> $x<T::Output> {
                let mut out = Vec::with_capacity(self.data.len());
                for (x, y) in rhs.data.into_iter().zip(self.data.into_iter()) {
                    out.push(x - y);
                }
                $x {
                    data: out,
                    rows: $m,
                    cols: $n,
                }
            }
        }

        impl<T: std::fmt::Display> std::fmt::Display for $x<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                for (i, s) in self.data.iter().enumerate() {
                    if i > 0 && i % ($n) == 0 {
                        write!(f, "\n")?;
                    }
                    write!(f, "{}, ", s)?;
                }
                write!(f, "")
            }
        }
    };
}

#[cfg(test)]
mod tests {
    new_matrix!(Matrix3x900, 100, 500);
    #[test]
    fn it_works() {
        let x = Matrix3x900::new([[3.5; 500]; 100]);
        println!("{}", x.clone().dot(x));
    }
}
