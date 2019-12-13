// https://atcoder.jp/contests/practice/tasks/practice_1

#[proconio::fastout]
fn main() {
    // use defmac::defmac;
    //
    // use std::io::{self, Read as _};
    //
    // let mut input = "".to_owned();
    // io::stdin().read_to_string(&mut input).unwrap();
    // let mut input = input.split_whitespace();
    // defmac!(read => input.next().unwrap().parse().unwrap());
    //
    // let (a, b, c, s): (u32, u32, u32, String) = (read!(), read!(), read!(), read!());

    use proconio::input;

    input! {
        a: u32,
        b: u32,
        c: u32,
        s: String,
    }

    println!("{} {}", a + b + c, s);
}
