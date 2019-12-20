// https://atcoder.jp/contests/abc120/tasks/abc120_d

use union_find::{QuickFindUf, UnionBySize, UnionFind as _};

use std::io::{self, Read as _};

// `proconio::fastout` does not accept `macro_rules!` until Rust 1.40.
macro_rules! macro_rules_hack {
    ($name:ident { $($tt:tt)* }) => {
        macro_rules! $name {
            $($tt)*
        }
    };
}

#[proconio::fastout]
fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules_hack!(read {
        ([$tt:tt; $n:expr]) => {
            (0..$n).map(|_| read!($tt)).collect::<Vec<_>>()
        };
        (($($tt:tt),+)) => {
            ($(read!($tt)),*)
        };
        (_1based) => {
            read!(usize) - 1
        };
        (_bytes) => {
            read!(String).into_bytes()
        };
        ($ty:ty) => {
            input.next().unwrap().parse::<$ty>().unwrap()
        };
    });

    let (n, m) = read!((usize, usize));
    let abs = read!([(_1based, _1based); m]);

    let mut uf = QuickFindUf::<UnionBySize>::new(n);
    let mut k = n * (n - 1) / 2;
    let mut ans_rev = vec![k];
    ans_rev.extend(abs.into_iter().rev().map(|(a, b)| {
        let p = uf.get(a).size() * uf.get(b).size();
        if uf.union(a, b) {
            k -= p;
        }
        k
    }));
    assert_eq!(ans_rev.pop(), Some(0));
    for x in ans_rev.into_iter().rev() {
        println!("{}", x);
    }
}
