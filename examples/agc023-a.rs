// https://atcoder.jp/contests/agc023/tasks/agc023_a
//
// 以下のクレートを使用。
//
// - `itertools-num`
// - `maplit`
// - `proconio`

use itertools_num::ItertoolsNum as _;
use maplit::hashmap;
use proconio::input;

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        r#as: [i64],
    }

    // `0`が一つが入ったcounterを`maplit::hashmap!`で作る。
    //
    // https://docs.rs/maplit/1/maplit/macro.hashmap.html
    let mut counter = hashmap!(0 => 1u64); // `0`が一つが入ったcounterを`maplit::hashmap!`で作る。

    // `<_ as itertools_num::ItertoolsNum>::cumsum`で作られた一次元累積和のイテレータを、`Vec`にせずにそのまま`for`文で回す。
    //
    // https://docs.rs/itertools-num/0.1/itertools_num/trait.ItertoolsNum.html#method.cumsum
    for sum in r#as.into_iter().cumsum() {
        *counter.entry(sum).or_insert(0) += 1;
    }

    let ans = counter.values().map(|v| v * (v - 1) / 2).sum::<u64>();
    println!("{}", ans);
}
