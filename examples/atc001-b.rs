// https://atcoder.jp/contests/atc001/tasks/unionfind_a

use petgraph::unionfind::UnionFind;
use proconio::source::{Readable, Source};
use proconio::{fastout, input};

use std::io::BufRead;

#[fastout]
fn main() {
    input! {
        n: usize,
        pabs: [(ZeroOne, usize, usize)],
    }

    let mut uf = UnionFind::new(n);
    for (p, a, b) in pabs {
        if p {
            let same = uf.find(a) == uf.find(b);
            println!("{}", if same { "Yes" } else { "No" });
        } else {
            uf.union(a, b);
        }
    }
}

enum ZeroOne {}

impl Readable for ZeroOne {
    type Output = bool;

    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> bool {
        u32::read(source) == 1
    }
}
