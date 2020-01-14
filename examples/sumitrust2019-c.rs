// https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_c

use defmac::defmac;
use fixedbitset::FixedBitSet;

use std::io::{self, Read};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
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

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
