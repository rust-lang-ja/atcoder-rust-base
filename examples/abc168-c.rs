// https://atcoder.jp/contests/abc168/tasks/abc168_c
//
// 以下のクレートを使用。
// - `num`
//     - `num-complex`
// - `proconio`

use num::Complex;
use proconio::input;
use std::f64::consts::PI;

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    // 座標として`num_complex::Complex<f64>`を使う。
    // `Complex::from_polar`でxとθから(x・cosθ, x・sinθ)を得ることができ、また`Complex::norm`で2点間の距離を出すことができる。
    //
    // https://docs.rs/num-complex/0.2.4/num_complex/struct.Complex.html
    // https://docs.rs/num-complex/0.2.4/num_complex/struct.Complex.html#method.from_polar
    // https://docs.rs/num-complex/0.2.4/num_complex/struct.Complex.html#method.norm

    let p1 = Complex::from_polar(&a, &(h * PI / 6.0 + m * PI / 360.0));
    let p2 = Complex::from_polar(&b, &(m * PI / 30.0));
    let ans = (p1 - p2).norm();
    println!("{}", ans);
}
