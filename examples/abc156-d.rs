// https://atcoder.jp/contests/abc156/tasks/abc156_d
//
// 以下のクレートを使用。
//
// - `num`
//     - `num-traits`
use num::One;
use proconio::input;

use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::ops::{Add, Div, Mul, MulAssign, Sub};
use std::str::FromStr;

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        n: Zp,
        a: Zp,
        b: Zp,
    }

    // `num_traits::pow`を使って2^nを求める。
    //
    // https://docs.rs/num-traits/0.2/num_traits/pow/fn.pow.html
    let ans = num::pow(Zp::new(2), n.repr) - binomial(n, a) - binomial(n, b) - Zp::new(1);
    println!("{}", ans);
}

fn binomial(n: Zp, k: Zp) -> Zp {
    let (mut numer, mut denom) = (n, Zp::new(1));
    for i in 2..=k.repr {
        numer *= n - Zp::new(i) + Zp::new(1);
        denom *= Zp::new(i);
    }
    numer / denom
}

const P: usize = 1_000_000_007;

#[derive(Debug, Clone, Copy)]
struct Zp {
    repr: usize,
}

impl Zp {
    fn new(val: usize) -> Self {
        Self { repr: val % P }
    }
}

impl FromStr for Zp {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        s.parse().map(Self::new)
    }
}

// `num_integer::pow`に必要。
impl One for Zp {
    fn one() -> Self {
        Self { repr: 1 }
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
        let repr = if self.repr < rhs.repr {
            P + self.repr - rhs.repr
        } else {
            self.repr - rhs.repr
        };
        Self { repr }
    }
}

impl Mul for Zp {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.repr * rhs.repr)
    }
}

impl MulAssign for Zp {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for Zp {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        // Fermatの小定理より。
        // `num_integer::Integer::extended_gcd`というのもあるのでこれを使っても良い。
        self * num::pow(rhs, P - 2)
    }
}

impl Display for Zp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.repr, fmt)
    }
}
