// https://atcoder.jp/contests/practice/tasks/practice_1

use text_io::read;

fn main() {
    let (a, b, c, s): (u32, u32, u32, String) = (read!(), read!(), read!(), read!());

    println!("{} {}", a + b + c, s);
}
