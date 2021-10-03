// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        mut plan: [(i32, i32, i32); n],  // Vec<(i32, i32, i32)>
    }
    plan.insert(0, (0, 0, 0));
    let yes = plan.windows(2).all(|w| {
        let (t0, x0, y0) = w[0];
        let (t1, x1, y1) = w[1];
        let time = t1 - t0;
        let dist = (x1 - x0).abs() + (y1 - y0).abs();
        dist <= time && time % 2 == dist % 2
    });
    println!("{}", if yes { "Yes" } else { "No" });
}
