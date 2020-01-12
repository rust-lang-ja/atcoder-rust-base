// https://atcoder.jp/contests/abc129/tasks/abc129_f

use defmac::defmac;
use derive_more::Display;
use ndarray::{array, Array2, LinalgScalar};
use num::{PrimInt, Unsigned};
use num_derive::{One, Zero};

use std::cell::Cell;
use std::cmp;
use std::io::{self, Read as _};
use std::ops::{Add, Div, Mul, Sub};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    defmac!(read => input.next().unwrap().parse().unwrap());

    let (l, a, b, m): (u64, u64, u64, u64) = (read!(), read!(), read!(), read!());

    MOD.with(|cell| cell.set(m));

    let count = |d| -> _ {
        let count =
            |above: u64| cmp::min(above.saturating_sub(a + 1) / b + u64::from(b < above), l);
        count(10u64.pow(d)) - count(10u64.pow(d - 1))
    };

    let ans = (1..=18).fold(array![[Z::new(0), Z::new(a), Z::new(1)]], |acc, d| {
        acc.dot(
            &array![
                [Z::new(10u64.pow(d)), Z::new(0), Z::new(0)],
                [Z::new(1), Z::new(1), Z::new(0)],
                [Z::new(0), Z::new(b), Z::new(1)],
            ]
            .matrix_power(count(d)),
        )
    })[(0, 0)];
    println!("{}", ans);
}

trait Array2Ext {
    fn matrix_power<E: PrimInt + Unsigned>(&self, exp: E) -> Self;
}

impl<S: LinalgScalar> Array2Ext for Array2<S> {
    fn matrix_power<E: PrimInt + Unsigned>(&self, exp: E) -> Self {
        let (mut base, mut exp, mut acc) = (self.clone(), exp, Self::eye(self.nrows()));
        while exp > E::zero() {
            if (exp & E::one()) == E::one() {
                acc = acc.dot(&base);
            }
            exp = exp / (E::one() + E::one());
            base = base.dot(&base);
        }
        acc
    }
}

thread_local! {
    static MOD: Cell<u64> = Cell::new(0);
}

#[derive(Zero, One, Display, Debug, Clone, Copy)]
struct Z(u64);

impl Z {
    fn new(val: u64) -> Self {
        Self(val % MOD.with(Cell::get))
    }
}

impl Add for Z {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0)
    }
}

impl Sub for Z {
    type Output = Self;

    fn sub(self, _: Self) -> Self {
        unreachable!("should not be performed")
    }
}

impl Mul for Z {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.0 * rhs.0)
    }
}

impl Div for Z {
    type Output = Self;

    fn div(self, _: Self) -> Self {
        unreachable!("should not be performed")
    }
}
