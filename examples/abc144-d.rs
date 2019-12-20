// https://atcoder.jp/contests/abc144/tasks/abc144_d

use defmac::defmac;

use std::f64::consts::PI;
use std::io::{self, Read as _};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    defmac!(read => input.next().unwrap().parse().unwrap());

    let (a, b, x): (f64, f64, f64) = (read!(), read!(), read!());

    let ans = 180.0 / PI
        * if x >= (a.powi(2) * b) / 2.0 {
            libm::atan2(2.0 * (a.powi(2) * b - x), a.powi(3))
        } else {
            PI / 2.0 - libm::atan2(2.0 * x, a * b.powi(2))
        };
    println!("{}", ans);
}
