// https://atcoder.jp/contests/atc002/tasks/atc002_b

use defmac::defmac;
use num::BigUint;

use std::io::{self, Read as _};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    defmac!(read => input.next().unwrap().parse().unwrap());

    let (n, m, p): (BigUint, BigUint, BigUint) = (read!(), read!(), read!());

    println!("{}", n.modpow(&p, &m));
}
