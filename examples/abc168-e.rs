// https://atcoder.jp/contests/abc168/tasks/abc168_e
//
// 以下のクレートを使用。
// - `maplit`
// - `num`
//     - `num-rational`
// - `proconio`

use maplit::hashmap;
use num::{rational::Ratio, One, Signed as _};
use proconio::input;
use std::{
    fmt,
    iter::Product,
    ops::{Add, Mul, Sub},
};

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        n: usize,
        abs: [(i64, i64); n],
    }

    // 有理数として`num_rational::Ratio<i64>`を使う。
    //
    // https://docs.rs/num-rational/0.2.4/num_rational/struct.Ratio.html

    let mut zeros = 0;
    let mut horz = 0;
    let mut vert = 0;
    let mut others = hashmap!();

    for (a, b) in abs {
        match (a.signum(), b.signum()) {
            (0, 0) => zeros += 1,
            (1, 0) | (-1, 0) => horz += 1,
            (0, 1) | (0, -1) => vert += 1,
            _ => *others.entry(Ratio::new(a, b)).or_insert(0) += 1usize,
        }
    }

    let ans = others
        .iter()
        .map(|(grad, &num1)| {
            // `Ratio::recip`で逆元を得ることができる。
            //
            // https://docs.rs/num-rational/0.2.4/num_rational/struct.Ratio.html#method.recip
            let num2 = *others.get(&-grad.recip()).unwrap_or(&0);
            if grad.is_negative() && num2 > 0 {
                Zp::unchecked(1)
            } else {
                xor_combinations(num1, num2)
            }
        })
        .product::<Zp>()
        * xor_combinations(horz, vert)
        + Zp::unchecked(zeros)
        - Zp::unchecked(1);
    println!("{}", ans);
}

fn xor_combinations(a: usize, b: usize) -> Zp {
    return pow2(a) + pow2(b) - Zp::unchecked(1);

    fn pow2(exp: usize) -> Zp {
        num::pow(Zp::unchecked(2), exp)
    }
}

const P: usize = 1_000_000_007;

#[derive(Clone, Copy, Debug)]
struct Zp {
    repr: usize,
}

impl Zp {
    fn new(val: usize) -> Self {
        Self { repr: val % P }
    }

    fn unchecked(repr: usize) -> Self {
        Self { repr }
    }
}

impl One for Zp {
    fn one() -> Self {
        Self::unchecked(1)
    }
}

impl fmt::Display for Zp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.repr, fmt)
    }
}

impl Add for Zp {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.repr + rhs.repr)
    }
}

impl Sub for Zp {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(P + self.repr - rhs.repr)
    }
}

impl Mul for Zp {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.repr * rhs.repr)
    }
}

impl Product for Zp {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::unchecked(1), Mul::mul)
    }
}
