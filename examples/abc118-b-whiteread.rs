// https://atcoder.jp/contests/abc118/tasks/abc118_b

use whiteread::Reader;

use std::ops::{BitAnd, BitOr};

fn main() {
    let mut rdr = Reader::from_stdin_naive();
    let (n, _) = rdr.p::<(usize, usize)>();
    let a = (0..n)
        .map(|_| {
            let k = rdr.p::<usize>();
            (0..k).map(|_| rdr.p::<usize>() - 1).collect()
        })
        .collect::<Vec<Vec<_>>>();

    let ans = a
        .into_iter()
        .map(|row| row.into_iter().map(|k| 1 << k).fold(0, BitOr::bitor))
        .fold(usize::max_value(), BitAnd::bitand)
        .count_ones();
    println!("{}", ans);
}
