// https://atcoder.jp/contests/abc120/tasks/abc120_d

use union_find::{QuickFindUf, UnionBySize, UnionFind as _};

#[proconio::fastout]
fn main() {
    // use std::io::{self, Read as _};
    //
    // let mut input = "".to_owned();
    // io::stdin().read_to_string(&mut input).unwrap();
    // let mut input = input.split_whitespace();
    // macro_rules! read {
    //     ([$t:tt; $n:expr]) => {
    //         (0..$n).map(|_| read!($t)).collect::<Vec<_>>()
    //     };
    //     (($($t:tt),+)) => {
    //         ($(read!($t)),*)
    //     };
    //     (_1based) => {
    //         read!(usize) - 1
    //     };
    //     (_bytes) => {
    //         read!(String).into_bytes()
    //     };
    //     ($ty:ty) => {
    //         input.next().unwrap().parse::<$ty>().unwrap()
    //     };
    // }
    //
    // let (n, m) = (read!((usize, usize)));
    // let abs = read!([(_1based, _1based); m]);

    use proconio::input;
    use proconio::marker::Usize1;

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
