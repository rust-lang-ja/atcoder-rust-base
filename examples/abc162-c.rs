// https://atcoder.jp/contests/abc162/tasks/abc162_c
//
// 以下のクレートを使用。
// - `num`
//     - `num-integer`
// - `itertools`
// - `proconio`

use itertools::iproduct;
use proconio::input;

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        k: u64,
    }

    // 三重ループを表わすイテレータを`itertools::iproduct!`で生み出す。
    // GCDは`num_integer::gcd`で`fold`することで求められる。
    //
    // https://docs.rs/itertools/0.9/itertools/macro.iproduct.html
    // https://docs.rs/num-integer/0.1/num_integer/fn.gcd.html
    let ans = iproduct!(1..=k, 1..=k, 1..=k)
        .map(|(a, b, c)| [a, b, c].iter().copied().fold(0, num::integer::gcd))
        .sum::<u64>();
    println!("{}", ans);
}
