// https://atcoder.jp/contests/agc026/tasks/agc026_c
//
// 以下のクレートを使用。
//
// - `either`
// - `itertools`
// - `proconio`
// - `rustc-hash`

use either::Either;
use itertools::Itertools as _;
use proconio::input;
use proconio::marker::Bytes;
use rustc_hash::FxHashMap;
use std::collections::HashMap;

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        n: usize,
        s: Bytes,
    }

    let first = pairs(&s[..n]);

    // `itertools::Itertools`により`.collect::<Vec<_>>()`を`.collect_vec`と書ける。
    // 結構微妙だけど`Itertools`を`use`してるなら考えてもいいかもしれない。
    //
    // https://docs.rs/itertools/0.8/itertools/trait.Itertools.html#method.collect_vec
    let second = pairs(&s[n..].iter().copied().rev().collect_vec());

    let ans = first
        .into_iter()
        .flat_map(|(p, n1)| second.get(&p).map(|n2| n1 * n2))
        .sum::<u64>();
    println!("{}", ans);
}

// `rustc-hash`が使えるのだが、この問題においてはデフォルトのハッシャーとの違いがよくわからない。
// というよりハッシュがボトルネックではない気がする。
//
// https://docs.rs/rustc-hash/1
fn pairs(half: &[u8]) -> FxHashMap<(Vec<u8>, Vec<u8>), u64> {
    let mut counter = HashMap::default();
    for bits in 0..1 << half.len() {
        // `<_ as itertools::Itertools>::partition_map`を用いることで2^{`half`}を表わすビット列に対して、
        // 具体的な(赤の列, 青の列) / (青の列, 赤の列)を求める。
        //
        // https://docs.rs/itertools/0.8/itertools/trait.Itertools.html#method.partition_map
        let pair = half.iter().enumerate().partition_map(|(i, &c)| {
            if bits >> i & 1 == 1 {
                Either::Left(c)
            } else {
                Either::Right(c)
            }
        });
        *counter.entry(pair).or_insert(0) += 1;
    }
    counter
}
