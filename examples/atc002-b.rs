// https://atcoder.jp/contests/atc002/tasks/atc002_b

use num::BigUint;
use proconio::input;

fn main() {
    input! {
        n: BigUint,
        m: BigUint,
        p: BigUint,
    }

    println!("{}", n.modpow(&p, &m));
}
