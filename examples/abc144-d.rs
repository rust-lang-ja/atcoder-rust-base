// https://atcoder.jp/contests/abc144/tasks/abc144_d

use std::f64::consts::PI;
use std::io::{self, Read};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read(() => (input.next().unwrap().parse().unwrap()));

    let (a, b, x): (f64, f64, f64) = (read!(), read!(), read!());

    let ans = 180.0 / PI
        * if x >= (a.powi(2) * b) / 2.0 {
            libm::atan2(2.0 * (a.powi(2) * b - x), a.powi(3))
        } else {
            PI / 2.0 - libm::atan2(2.0 * x, a * b.powi(2))
        };
    println!("{}", ans);
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
