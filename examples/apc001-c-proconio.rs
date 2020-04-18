// https://atcoder.jp/contests/apc001/tasks/apc001_c
//
// 以下のクレートを使用。
// - `proconio`

use proconio::source::line::LineSource;
use std::{cmp, io, panic, process};

fn main() {
    panic::set_hook(Box::new(|_| {
        // 変なクエリを吐いて`RE`させ、`TLE`を回避
        println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    }));

    // `proconio::source::line::LineSource`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/source/line/struct.LineSource.html
    let stdin = io::stdin();
    let mut stdin = LineSource::new(stdin.lock());
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input!(n: usize);

    let mut query = |i: usize| -> _ {
        println!("{}", i);
        input!(s: String);
        match &*s {
            "Vacant" => process::exit(0),
            "Male" => false,
            "Female" => true,
            _ => unreachable!(),
        }
    };

    let first = query(0);
    let last = query(n - 1);

    // Nは小さいので`Vec`を作って`<[_]>::binary_search`を悪用
    (1..n)
        .collect::<Vec<_>>()
        .binary_search_by(|&i| {
            let query = query(i);
            if (i % 2 == 0) == (query == first) || ((n - i - 1) % 2 == 0) != (query == last) {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Greater
            }
        })
        .unwrap();
    unreachable!();
}
