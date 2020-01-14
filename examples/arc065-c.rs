// https://atcoder.jp/contests/arc065/tasks/arc065_a

use lazy_static::lazy_static;
use regex::bytes::Regex;

use std::io::{self, Read as _};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read {
        (_bytes)            => { read!(String).into_bytes() };
        ([$tt:tt])          => { read!([$tt; read!(usize)]) };
        ([$tt:tt; $n:expr]) => { (0..$n).map(|_| read!($tt)).collect::<Vec<_>>() };
        (($($tt:tt),+))     => { ($(read!($tt)),*) };
        ($ty:ty)            => { input.next().unwrap().parse::<$ty>().unwrap() };
    }

    let s = read!(_bytes);

    lazy_static! {
        static ref R: Regex = Regex::new(r"\A(dream(er)?|eraser?)*\z").unwrap();
    };
    println!("{}", if R.is_match(&s) { "YES" } else { "NO" });
}
