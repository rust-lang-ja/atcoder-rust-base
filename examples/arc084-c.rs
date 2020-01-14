// https://atcoder.jp/contests/arc084/tasks/arc084_a

use superslice::Ext as _;

use std::io::{self, Read as _};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read {
        ([$tt:tt; $n:expr]) => { (0..$n).map(|_| read!($tt)).collect::<Vec<_>>() };
        (($($tt:tt),+))     => { ($(read!($tt)),*) };
        ($ty:ty)            => { input.next().unwrap().parse::<$ty>().unwrap() };
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
