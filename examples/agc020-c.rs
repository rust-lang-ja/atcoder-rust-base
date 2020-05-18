// https://atcoder.jp/contests/agc020/tasks/agc020_c
//
// 以下のクレートを使用。
// - `bitset-fixed`
// - `proconio`

use bitset_fixed::BitSet;
use proconio::input;

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        r#as: [usize],
    }

    // `bitset-fixed`のREADMEにある解法。

    let sum = r#as.iter().sum::<usize>();

    // サイズ`sum + 1`のbit setに`fixed_bitset::BitSet`を使う。
    // これは`fixedbitset::FixedBitSet`と違いビットシフトが可能。
    // (`bitvec`クレートだと可能。提案しておけばよかったか..)
    //
    // https://docs.rs/bitset-fixed/0.1/bitset_fixed/struct.BitSet.html
    let mut dp = BitSet::new(sum + 1);
    dp.set(0, true);
    for a in r#as {
        dp |= &(&dp << a);
    }

    let ans = ((sum + 1) / 2..).find(|&i| dp[i]).unwrap();
    println!("{}", ans);
}
