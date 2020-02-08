// https://atcoder.jp/contests/abc122/tasks/abc122_c
//
// 以下のクレートを使用。
//
// - `itertools-num`
// - `proconio`

use itertools_num::ItertoolsNum as _;
use proconio::marker::{Bytes, Usize1};
use proconio::{fastout, input};

use std::iter;

// `#[proconio::fastout]`で`println!`を置き換える。
//
// https://docs.rs/proconio-derive/0.1/proconio_derive/attr.fastout.html
#[fastout]
fn main() {
    // `proconio::input!`では使わない値を`_`とすることができる。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        _: usize,
        q: usize,
        s: Bytes,
        lrs: [(Usize1, Usize1); q],
    }

    // `<_ as itertools_num::ItertoolsNum>::cumsum`で作られた一次元累積和のイテレータを、先頭に`0`を挿入した上で`Vec<_>`にする。
    //
    // https://docs.rs/itertools-num/0.1/itertools_num/trait.ItertoolsNum.html#method.cumsum
    let cumsum = iter::once(0)
        .chain(s.windows(2).map(|w| (w == b"AC").into()))
        .cumsum()
        .collect::<Vec<u32>>();

    for (l, r) in lrs {
        println!("{}", cumsum[r] - cumsum[l]);
    }
}
