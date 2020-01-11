// https://atcoder.jp/contests/abc121/tasks/abc121_b

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: i32,
        b: [i32; m],
        a: [[i32; m]; n],
    }

    let ans = a
        .into_iter()
        .filter(|a| a.iter().zip(&b).map(|(a, b)| a * b).sum::<i32>() + c > 0)
        .count();
    println!("{}", ans);
}
