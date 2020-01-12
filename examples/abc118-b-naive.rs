// https://atcoder.jp/contests/abc118/tasks/abc118_b

use std::io::{self, Read as _};
use std::ops::{BitAnd, BitOr};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read {
        ([$tt:tt; $n:expr]) => {
            (0..$n).map(|_| read!($tt)).collect::<Vec<_>>()
        };
        (($($tt:tt),+)) => {
            ($(read!($tt)),*)
        };
        (_1based) => {
            read!(usize) - 1
        };
        ($ty:ty) => {
            input.next().unwrap().parse::<$ty>().unwrap()
        };
    }

    let (n, _) = read!((usize, usize));
    let a = (0..n)
        .map(|_| read!([_1based; read!(usize)]))
        .collect::<Vec<_>>();

    let ans = a
        .into_iter()
        .map(|row| row.into_iter().map(|k| 1 << k).fold(0, BitOr::bitor))
        .fold(usize::max_value(), BitAnd::bitand)
        .count_ones();
    println!("{}", ans);
}