// https://atcoder.jp/contests/arc065/tasks/arc065_a

use lazy_static::lazy_static;
use proconio::input;
use proconio::marker::Bytes;
use regex::bytes::Regex;

fn main() {
    input! {
        s: Bytes,
    }

    lazy_static! {
        static ref R: Regex = Regex::new(r"\A(dream(er)?|eraser?)*\z").unwrap();
    };
    println!("{}", if R.is_match(&s) { "YES" } else { "NO" });
}
