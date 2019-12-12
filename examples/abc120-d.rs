// https://atcoder.jp/contests/abc120/tasks/abc120_d

use proconio::marker::Usize1;
use proconio::{fastout, input};
use union_find::{QuickFindUf, UnionBySize, UnionFind as _};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abs: [(Usize1, Usize1); m],
    }

    let mut u = QuickFindUf::<UnionBySize>::new(n);
    let mut k = n * (n - 1) / 2;
    let mut r = vec![k];
    r.extend(abs.into_iter().rev().map(|(a, b)| {
        let p = u.get(a).size() * u.get(b).size();
        if u.union(a, b) {
            k -= p;
        }
        k
    }));
    assert_eq!(r.pop(), Some(0));
    for r in r.into_iter().rev() {
        println!("{}", r);
    }
}
