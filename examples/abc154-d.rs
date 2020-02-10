// https://atcoder.jp/contests/abc154/tasks/abc154_d
//
// 以下のクレートを使用。
//
// - `itertools-num`
// - `ordered-float`
// - `proconio`

use itertools_num::ItertoolsNum as _;
use ordered_float::NotNan;
use proconio::input;

use std::iter;

// `NotNan<f64>: FromStr`であり、四則演算の右辺に`f64`が許されているので`NotNan`のリテラルが必要になることは少ない。
// 必要な場合このようなマクロや関数等を用意しておくと見た目は軽くなる。
macro_rules! notnan(($lit:literal) => (NotNan::new($lit).unwrap()));

fn main() {
    // `proconio::input!`は(オリジナルの`input!`もそうだが)`FromStr`を実装する値なら何でも読める。
    // ここでは{ p_i }を`ordered_float::NotNan<f64>`として受け取る。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    // https://docs.rs/ordered-float/1/ordered_float/struct.NotNan.html
    input! {
        n: usize,
        k: usize,
        ps: [NotNan<_>; n],
    }

    // 先頭に`0`を挿入した一次元累積和のイテレータを`<_ as itertools_num::ItertoolsNum>::cumsum`で作り、`Vec<_>`にする。
    //
    // https://docs.rs/itertools-num/0.1/itertools_num/trait.ItertoolsNum.html#method.cumsum
    let ans = iter::once(notnan!(0.0))
        .chain(ps.into_iter().map(|p| (p + 1.0) / 2.0))
        .cumsum()
        .collect::<Vec<NotNan<_>>>()
        .windows(k + 1)
        .map(|w| w[w.len() - 1] - w[0])
        .max()
        .unwrap();
    println!("{}", ans);
}
