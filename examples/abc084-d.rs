// https://atcoder.jp/contests/abc084/tasks/abc084_d

use itertools_num::ItertoolsNum as _;
use primal::Sieve;

use std::io::{self, Read as _};

// `proconio::fastout` does not accept `macro_rules!` until Rust 1.40.
macro_rules! macro_rules_hack {
    ($name:ident { $($tt:tt)* }) => {
        macro_rules! $name {
            $($tt)*
        }
    };
}

#[proconio::fastout]
fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules_hack!(read {
        ([$tt:tt; $n:expr]) => {
            (0..$n).map(|_| read!($tt)).collect::<Vec<_>>()
        };
        (($($tt:tt),+)) => {
            ($(read!($tt)),*)
        };
        (_1based) => {
            read!(usize) - 1
        };
        (_bytes) => {
            read!(String).into_bytes()
        };
        ($ty:ty) => {
            input.next().unwrap().parse::<$ty>().unwrap()
        };
    });

    let q = read!(usize);
    let lrs = read!([(usize, usize); q]);

    let hi = lrs.iter().map(|&(_, r)| r).max().unwrap();
    let sieve = Sieve::new(hi);
    let nums = (0..=hi)
        .map(|k| u32::from(sieve.is_prime(k) && sieve.is_prime((k + 1) / 2)))
        .cumsum()
        .collect::<Vec<u32>>();
    for (l, r) in lrs {
        println!("{}", nums[r] - nums[l - 1]);
    }
}
