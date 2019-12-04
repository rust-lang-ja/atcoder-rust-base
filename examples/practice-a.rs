// https://atcoder.jp/contests/practice/tasks/practice_1

use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        s: String,
    }

    println!("{} {}", a + b + c, s);
}
