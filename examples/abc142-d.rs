// https://atcoder.jp/contests/abc142/tasks/abc142_d

use primal::Sieve;

use std::cmp::max;
use std::collections::HashSet;

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
    // let (n, m) = read!((usize, usize));

    use proconio::input;

    input! {
        a: usize,
        b: usize,
    }

    // サンプルケースでしか試してないので嘘かもしれない。

    let sieve = Sieve::new(num_integer::sqrt(max(a, b)));
    let bases = |k| -> HashSet<_> {
        sieve
            .factor(k)
            .unwrap()
            .into_iter()
            .map(|(p, _)| p)
            .collect()
    };
    println!("{}", (&bases(a) & &bases(b)).len() + 1);
}
