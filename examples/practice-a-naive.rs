// https://atcoder.jp/contests/practice/tasks/practice_1

use std::io::{self, Read as _};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(() => (input.next().unwrap().parse().unwrap()));

    let (a, b, c, s): (u32, u32, u32, String) = (read!(), read!(), read!(), read!());

    println!("{} {}", a + b + c, s);
}
