// https://atcoder.jp/contests/atc002/tasks/atc002_b

use num::BigUint;

use std::io::{self, Read};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read(() => (input.next().unwrap().parse().unwrap()));

    let (n, m, p): (BigUint, BigUint, BigUint) = (read!(), read!(), read!());

    println!("{}", n.modpow(&p, &m));
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
