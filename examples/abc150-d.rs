// https://atcoder.jp/contests/abc150/tasks/abc150_d

use itertools::Itertools as _;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    if !a.iter().copied().map(usize::trailing_zeros).all_equal() {
        println!("0");
        return;
    }

    let x0 = a.into_iter().fold(1, num::integer::lcm) / 2;
    let ans = (m + x0) / (2 * x0);
    println!("{}", ans);
}
