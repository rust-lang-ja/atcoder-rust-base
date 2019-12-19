// https://atcoder.jp/contests/arc084/tasks/arc084_a

use superslice::Ext as _;

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
    // let n = read!(n);
    // let (mut a, b, mut c) = read!(([u32; n], [u32; n], [u32; n]));

    use proconio::input;

    input! {
        n: usize,
        mut a: [u32; n],
        b: [u32; n],
        mut c: [u32; n],
    }

    a.sort();
    c.sort();
    let ans = b
        .into_iter()
        .map(|b| a.lower_bound(&b) * (n - c.upper_bound(&b)))
        .sum::<usize>();
    println!("{}", ans);
}
