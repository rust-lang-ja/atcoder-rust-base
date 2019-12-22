// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_c

use defmac::defmac;
use fixedbitset::FixedBitSet;

use std::io::{self, Read as _};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    defmac!(read => input.next().unwrap().parse().unwrap());

    let x: usize = read!();

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
