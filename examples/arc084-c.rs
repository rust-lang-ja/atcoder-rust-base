// https://atcoder.jp/contests/arc084/tasks/arc084_a

use superslice::Ext as _;

use std::io::{self, Read};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read {
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    }

    let n = read!(usize);
    let (mut a, b, mut c) = read!(([u32; n], [u32; n], [u32; n]));

    a.sort();
    c.sort();
    let ans = b
        .into_iter()
        .map(|b| a.lower_bound(&b) * (n - c.upper_bound(&b)))
        .sum::<usize>();
    println!("{}", ans);
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
