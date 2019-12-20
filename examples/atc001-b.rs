// https://atcoder.jp/contests/atc001/tasks/unionfind_a

use petgraph::unionfind::UnionFind;

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

    let (n, q) = read!((usize, usize));
    let pabs = read!([(u8, usize, usize); q]);

    let mut uf = UnionFind::new(n);
    for (p, a, b) in pabs {
        if p == 1 {
            let same = uf.find(a) == uf.find(b);
            println!("{}", if same { "Yes" } else { "No" });
        } else {
            uf.union(a, b);
        }
    }
}
