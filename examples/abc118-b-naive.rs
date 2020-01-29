// https://atcoder.jp/contests/abc118/tasks/abc118_b

use std::io::{self, Read};
use std::ops::{BitAnd, BitOr};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read {
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
        ({ Usize1 }) => {
            read!(usize) - 1
        };
    }

    let (n, _) = read!((usize, usize));
    let a = read!([[{ Usize1 }]; n]);

    let ans = a
        .into_iter()
        .map(|row| row.into_iter().map(|k| 1 << k).fold(0, BitOr::bitor))
        .fold(usize::max_value(), BitAnd::bitand)
        .count_ones();
    println!("{}", ans);
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
