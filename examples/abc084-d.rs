// https://atcoder.jp/contests/abc084/tasks/abc084_d

use itertools_num::ItertoolsNum as _;
use primal::Sieve;

#[proconio::fastout]
fn main() {
    // use std::io::{self, Read as _};
    //
    // let mut input = "".to_owned();
    // io::stdin().read_to_string(&mut input).unwrap();
    // let mut input = input.split_whitespace();
    // macro_rules! read {
    //     ([$t:tt; $n:expr]) => {
    //         (0..$n).map(|_| read!($t)).collect::<Vec<_>>()
    //     };
    //     (($($t:tt),+)) => {
    //         ($(read!($t)),*)
    //     };
    //     (_1based) => {
    //         read!(usize) - 1
    //     };
    //     (_bytes) => {
    //         read!(String).into_bytes()
    //     };
    //     ($ty:ty) => {
    //         input.next().unwrap().parse::<$ty>().unwrap()
    //     };
    // }
    //
    // let q = read!(usize);
    // let lrs = read!([(usize, usize); q]);

    use proconio::input;

    input! {
        q: usize,
        lrs: [(usize, usize); q],
    }

    // サンプルケースでしか試してないので嘘かもしれない。

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
