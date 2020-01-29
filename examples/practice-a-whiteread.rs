// https://atcoder.jp/contests/practice/tasks/practice_1

use whiteread::Reader;

fn main() {
    let mut rdr = Reader::from_stdin_naive();

    let (a, b, c, s) = rdr.p::<(u32, u32, u32, String)>();

    println!("{} {}", a + b + c, s);
}
