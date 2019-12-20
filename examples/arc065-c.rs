// https://atcoder.jp/contests/arc065/tasks/arc065_a

use lazy_static::lazy_static;
use regex::bytes::Regex;

use std::io::{self, Read as _};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();

    let s = input.trim_end().as_bytes().to_owned();

    lazy_static! {
        static ref R: Regex = Regex::new(r"\A(dream(er)?|eraser?)*\z").unwrap();
    };
    println!("{}", if R.is_match(&s) { "YES" } else { "NO" });
}
