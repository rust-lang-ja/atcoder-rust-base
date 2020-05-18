// https://atcoder.jp/contests/judge-update-202004/tasks/judge_update_202004_b
//
// 以下のクレートを使用。
// - `itertools`
// - `proconio`

use itertools::Itertools as _;
use proconio::{fastout, input};

// `#[proconio::fastout]`で標準出力を高速化する。
//
// https://docs.rs/proconio-derive/0.1/proconio_derive/attr.fastout.html
#[fastout]
fn main() {
    // `proconio::input!`で入力を読む。
    //
    // https://docs.rs/proconio/0.3/proconio/macro.input.html
    input! {
        xcs: [(u32, char)],
    }

    // `itertools::Itertools::sorted_by_key`で昇順かつ`R`, `B`の順に並び換えた(色, 書かれた整数)のイテレータを得る。
    // その実装は単純に`Vec<_>`にしたあと`.sort_by_key(..)`して`.into_iter()`しているだけ。
    //
    // https://docs.rs/itertools/0.9/itertools/trait.Itertools.html#method.sorted_by_key
    for (x, _) in xcs.into_iter().sorted_by_key(|&(x, c)| (c == 'B', x)) {
        println!("{}", x);
    }
}
