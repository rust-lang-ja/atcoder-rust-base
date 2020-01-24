// https://atcoder.jp/contests/arc084/tasks/arc084_a

use proconio::input;
use superslice::Ext as _;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
        b: [u32; n],
        mut c: [u32; n],
    }

    a.sort();
    c.sort();
    let ans = b
        .into_iter()
        .map(|b| a.lower_bound(&b) * (n - c.upper_bound(&b)))
        .sum::<usize>();
    println!("{}", ans);
}
