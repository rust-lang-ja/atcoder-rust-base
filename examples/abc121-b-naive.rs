// https://atcoder.jp/contests/abc121/tasks/abc121_b

use std::io::{self, Read as _};

#[allow(clippy::many_single_char_names)]
fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read {
        ([$tt:tt])          => { read!([$tt; read!(usize)]) };
        ([$tt:tt; $n:expr]) => { (0..$n).map(|_| read!($tt)).collect::<Vec<_>>() };
        (($($tt:tt),+))     => { ($(read!($tt)),*) };
        ($ty:ty)            => { input.next().unwrap().parse::<$ty>().unwrap() };
    }

    let (n, m, c) = read!((usize, usize, i32));
    let (b, a) = read!(([i32; m], [[i32; m]; n]));

    let ans = a
        .into_iter()
        .filter(|a| a.iter().zip(&b).map(|(a, b)| a * b).sum::<i32>() + c > 0)
        .count();
    println!("{}", ans);
}
