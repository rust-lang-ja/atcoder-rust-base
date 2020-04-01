// https://atcoder.jp/contests/abc160/tasks/abc160_e
//
// 以下のクレートを使用。
// - `itertools`
// - `proconio`

use itertools::Itertools as _;
use proconio::input;

use std::cmp::{self, Reverse};

fn main() {
    // `proconio::input!`はオリジナルの`input!`とは違い、`mut $ident`の形式で入力を読むことができる。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        mut ps: [u64; a],
        mut qs: [u64; b],
        mut rs: [u64; c],
    }

    ps.sort_unstable_by_key(|&p| Reverse(p));
    qs.sort_unstable_by_key(|&q| Reverse(q));
    rs.sort_unstable_by_key(|&r| Reverse(r));

    // `itertools::Itertools::kmerge_by`で降順のままでマージする。
    //
    // https://docs.rs/itertools/0.9/itertools/trait.Itertools.html#method.kmerge_by
    let ans = vec![&ps[..x], &qs[..y], &rs[..cmp::min(x + y, c)]]
        .into_iter()
        .kmerge_by(|v1, v2| v1 > v2)
        .take(x + y)
        .sum::<u64>();
    println!("{}", ans);
}
