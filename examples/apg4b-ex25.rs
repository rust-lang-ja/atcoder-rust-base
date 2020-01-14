// https://atcoder.jp/contests/APG4b/tasks/APG4b_bx

use fixedbitset::FixedBitSet;
use itertools::Itertools as _;

use std::io::{self, Read as _};

#[allow(clippy::many_single_char_names)]
fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    #[rustfmt::skip]
    macro_rules! read {
        ([$tt:tt])          => { read!([$tt; read!(usize)]) };
        ([$tt:tt; $n:expr]) => { (0..$n).map(|_| read!($tt)).collect::<Vec<_>>() };
        ($ty:ty)            => { input.next().unwrap().parse::<$ty>().unwrap() };
    }

    let a = read!([usize]);
    let b = read!([usize]);
    let arg0 = read!(String);
    let args = read!([usize; if arg0 == "subtract" { 1 } else { 0 }]);

    let (a, b) = (a.into_iter().collect(), b.into_iter().collect());

    print_set(&match (&*arg0, &*args) {
        ("intersection", []) => intersection(&a, &b),
        ("union_set", []) => union_set(&a, &b),
        ("symmetric_diff", []) => symmetric_diff(&a, &b),
        ("subtract", &[x]) => subtract(a, x),
        ("increment", []) => increment(&a),
        ("decrement", []) => decrement(&a),
        _ => unreachable!(),
    });
}

fn print_set(set: &FixedBitSet) {
    println!("{}", (0..50).filter(|&i| set[i]).format(" "));
}

fn intersection(a: &FixedBitSet, b: &FixedBitSet) -> FixedBitSet {
    a & b
}

fn union_set(a: &FixedBitSet, b: &FixedBitSet) -> FixedBitSet {
    a | b
}

fn symmetric_diff(a: &FixedBitSet, b: &FixedBitSet) -> FixedBitSet {
    a ^ b
}

fn subtract(mut a: FixedBitSet, x: usize) -> FixedBitSet {
    // > xは存在することが保証される。
    a.set(x, false);
    a
}

fn increment(a: &FixedBitSet) -> FixedBitSet {
    a.ones().map(|x| (x + 1) % 50).collect()
}

fn decrement(a: &FixedBitSet) -> FixedBitSet {
    a.ones().map(|x| (x + 49) % 50).collect()
}
