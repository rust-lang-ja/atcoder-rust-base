// https://atcoder.jp/contests/abc057/tasks/abc057_b

use std::io::{self, Read};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read {
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
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

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
