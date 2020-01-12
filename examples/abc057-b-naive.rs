// https://atcoder.jp/contests/abc057/tasks/abc057_b

use std::io::{self, Read as _};

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
        ($ty:ty) => {
            input.next().unwrap().parse::<$ty>().unwrap()
        };
    }

    let (n, m) = read!((usize, usize));
    let (abs, cds) = read!(([(i64, i64); n], [(i64, i64); m]));

    for (a, b) in abs {
        let j = (0..m)
            .min_by_key(|&j| {
                let (c, d) = cds[j];
                (a - c).abs() + (b - d).abs()
            })
            .unwrap();
        println!("{}", j + 1);
    }
}