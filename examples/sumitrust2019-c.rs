// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_c

use fixedbitset::FixedBitSet;
use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut dp = FixedBitSet::with_capacity(x + 105);
    dp.insert(0);
    for i in 0..=x.saturating_sub(100) {
        if dp[i] {
            // `insert_range` does not accept `RangeInclusive<usize>`.
            dp.insert_range(i + 100..i + 106);
        }
    }
    println!("{}", u32::from(dp[x]));
}
