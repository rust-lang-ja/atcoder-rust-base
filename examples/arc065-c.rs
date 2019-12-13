// https://atcoder.jp/contests/arc065/tasks/arc065_a

use lazy_static::lazy_static;
use regex::bytes::Regex;

#[proconio::fastout]
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
    // let s = read!(_bytes);

    use proconio::input;
    use proconio::marker::Bytes;

    input! {
        s: Bytes,
    }

    lazy_static! {
        static ref R: Regex = Regex::new(r"\A(dream(er)?|eraser?)*\z").unwrap();
    };
    println!("{}", if R.is_match(&s) { "YES" } else { "NO" });
}
