// https://atcoder.jp/contests/abc166/tasks/abc166_b
//
// 以下のクレートを使用。
// - `itertools`
// - `proconio`

use itertools::Itertools as _;
use proconio::{input, marker::Usize1};

fn main() {
    // `proconio::input!`には`n: usize, xs: [T; n]`のかわりに`xs: [T]`と書ける機能がある。
    // この機能を用いることで今回のような入力をシンプルにできる。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        n: usize,
        ass: [[Usize1]],
    }

    // `Itertools::unique`で要素の重複が省かれたイテレータが手に入る。
    //
    // https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.unique
    let ans = n - ass.into_iter().flatten().unique().count();

    // あるいは`fixedbitset::FixedBitSet`を使っても良い。
    // `FixedBitSet`は`FromIterator<usize>`であり、`count_ones`という「`1`」を数えるメソッドを持つ。
    //
    // https://docs.rs/fixedbitset/0.2.0/fixedbitset/struct.FixedBitSet.html
    // https://docs.rs/fixedbitset/0.2.0/fixedbitset/struct.FixedBitSet.html#method.count_ones
    //use fixedbitset::FixedBitSet;
    //let ans = n - ass
    //    .into_iter()
    //    .flatten()
    //    .collect::<FixedBitSet>()
    //    .count_ones(..);

    println!("{}", ans);
}
