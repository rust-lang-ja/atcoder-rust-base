// https://atcoder.jp/contests/abc118/tasks/abc118_b

use text_io::read;

use std::ops::{BitAnd, BitOr};

fn main() {
    let (n, _): (usize, usize) = (read!(), read!());
    let a = (0..n)
        .map(|_| {
            (0..read!())
                .map(|_| {
                    let a: usize = read!();
                    a - 1
                })
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let ans = a
        .into_iter()
        .map(|row| row.into_iter().map(|k| 1 << k).fold(0, BitOr::bitor))
        .fold(usize::max_value(), BitAnd::bitand)
        .count_ones();
    println!("{}", ans);
}
