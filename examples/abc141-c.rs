// https://atcoder.jp/contests/abc141/tasks/abc141_c

use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        a: [Usize1; q],
    }

    let mut correct = vec![0; n];
    a.into_iter().for_each(|a| correct[a] += 1);

    for correct in correct {
        let p = k + correct > q;
        println!("{}", if p { "Yes" } else { "No" });
    }
}
