// https://atcoder.jp/contests/abc165/tasks/abc165_c
//
// 以下のクレートを使用。
// - `itertools`
// - `proconio`

use itertools::Itertools as _;
use proconio::{input, marker::Usize1};

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        n: usize,
        m: usize,
        abcds: [(Usize1, Usize1, usize, u64)],
    }

    // `Itertools::combinations_with_replacement`で広義単調増加なAをすべて列挙する。
    // Pythonの同名な関数と同じ。
    //
    // https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.combinations_with_replacement
    let ans = (0..m)
        .combinations_with_replacement(n)
        .map(|arr| {
            abcds
                .iter()
                .filter(|&&(a, b, c, _)| arr[b] - arr[a] == c)
                .map(|(_, _, _, d)| d)
                .sum::<u64>()
        })
        .max()
        .unwrap();
    println!("{}", ans);
}
