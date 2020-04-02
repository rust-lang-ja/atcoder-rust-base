// https://atcoder.jp/contests/abc118/tasks/abc118_b

use proconio::input;
use proconio::marker::Usize1;

use std::ops::{BitAnd, BitOr};

fn main() {
    input! {
        n: usize,
        _: usize,
        a: [[Usize1]; n],
    }

    let ans = a
        .into_iter()
        .map(|row| row.into_iter().map(|k| 1 << k).fold(0, BitOr::bitor))
        .fold(usize::max_value(), BitAnd::bitand)
        .count_ones();
    println!("{}", ans);
}
