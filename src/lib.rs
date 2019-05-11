#[allow(unused_imports)]
use rayon::prelude::*;

#[allow(unused_macros)]
macro_rules! new_matrix {
    ($x: ident, $n: expr, $m: expr) => {
        #[derive(Debug)]
        struct $x<T> {
            pub data: Vec<T>,
            rows: usize,
            cols: usize,
        }
        #[allow(dead_code)]
        impl<T: std::ops::Add + Clone> $x<T> {
            fn new(data: [[T; $m]; $n]) -> $x<T> {
                let mut v: Vec<T> = Vec::with_capacity($m * $n);
                data.iter().for_each(|d| v.extend(d.to_vec()));
                $x {
                    data: v,
                    rows: $m,
                    cols: $n,
                }
            }
        }
        impl<T: std::ops::Add + Clone> std::ops::Add<$x<T>> for $x<T> {
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
        impl<T: std::fmt::Display> std::fmt::Display for $x<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                for (i, s) in self.data.iter().enumerate() {
                    if i > 0 && i % ($n + 1) == 0 {
                        write!(f, "\n")?;
                    }
                    write!(f, "{}, ", s)?;
                }
                write!(f, "h")
            }
        }
    };
}

#[cfg(test)]
mod tests {
    new_matrix!(Matrix3x4, 3, 4);
    #[test]
    fn it_works() {
        let x = Matrix3x4::new([
            [1.4, 1.5, 1.3, 1.2],
            [1.5, 1.3, 1.4, 1.9],
            [1.5, 1.3, 1.4, 1.9],
        ]);
        println!("{}", x);
    }
}
