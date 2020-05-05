// https://atcoder.jp/contests/abc165/tasks/abc165_b
//
// 以下のクレートを使用。
// - `itertools`
// - `proconio`

use proconio::input;

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        x: u64,
    }

    // `itertools::iterate`は`std::iter::successors`の`Some`固定版。
    // 「X円一歩手前」で打ち切ったものを`count`するとちょうど答えになる。
    //
    // https://docs.rs/itertools/0.9.0/itertools/fn.iterate.html
    let ans = itertools::iterate(100, |m| m + m / 100)
        .take_while(|&m| m < x)
        .count();

    println!("{}", ans);
}
