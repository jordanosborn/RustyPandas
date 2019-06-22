#![allow(unused_macros)]
#![deny(clippy::all, clippy::pedantic)]

pub mod calculus;
pub mod constants;
pub mod fitting;
pub mod functions;
pub mod statistics;
pub mod linalg;
pub mod types;

#[macro_export]
macro_rules! equality_test {
    ($name: ident, $f: expr, $( $var:expr ),* => $eq: expr) => {
        #[test]
        fn $name() {
            assert_eq!($eq, $f($($var),*));
        }
    };
}

#[macro_export]
macro_rules! abs_approx_test {
    ($name: ident, $f: expr, $( $var:expr ),* => $eq: expr , $tol: expr) => {
        #[test]
        fn $name() {
            let out = $f($($var),*);
            assert!((out - $eq).abs() < $tol);
        }
    };
    ($name: ident, $f: expr, $( $var:expr ),* => $eq: expr) => {
        #[test]
        fn $name() {
            let out = $f($($var),*);
            assert!((out - $eq).abs() < 0.0001);
        }
    };
}

#[macro_export]
macro_rules! inequality_test {
    ($name: ident, $f: expr, $( $var:expr ),* => $eq: expr) => {
        #[test]
        fn $name() {
            assert_ne!($eq, $f($($var),*));
        }
    };
}

#[macro_export]
macro_rules! panic_test {
    ($name: ident, $f: expr, $( $var:expr ),*) => {
        #[test]
        #[should_panic]
        fn $name() {
            $f($($var),*);
        }
    };
}
