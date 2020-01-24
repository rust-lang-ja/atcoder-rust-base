// https://atcoder.jp/contests/abc144/tasks/abc144_d

use proconio::input;

use std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    }

    let ans = 180.0 / PI
        * if x >= (a.powi(2) * b) / 2.0 {
            libm::atan2(2.0 * (a.powi(2) * b - x), a.powi(3))
        } else {
            PI / 2.0 - libm::atan2(2.0 * x, a * b.powi(2))
        };
    println!("{}", ans);
}
