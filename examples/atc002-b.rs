// https://atcoder.jp/contests/atc002/tasks/atc002_b

use num::BigUint;

#[proconio::fastout]
fn main() {
    // use defmac::defmac;
    //
    // use std::io::{self, Read as _};
    //
    // let mut input = "".to_owned();
    // io::stdin().read_to_string(&mut input).unwrap();
    // let mut input = input.split_whitespace();
    // defmac!(read => input.next().unwrap().parse().unwrap());
    //
    // let n: BigUint = read!();
    // let m: BigUint = read!();
    // let p: BigUint = read!();

    use proconio::input;

    input! {
        n: BigUint,
        m: BigUint,
        p: BigUint,
    }

    println!("{}", n.modpow(&p, &m));
}
