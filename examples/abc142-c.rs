// https://atcoder.jp/contests/abc142/tasks/abc142_c

use itertools::Itertools as _;
use proconio::input;
use proconio::marker::Isize1;
use superslice::Ext2 as _;

fn main() {
    input! {
        mut a: [Isize1],
    }

    a.invert_permutation();
    println!("{}", a.iter().map(|a| a + 1).format(" "));
}
