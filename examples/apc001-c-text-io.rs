// https://atcoder.jp/contests/apc001/tasks/apc001_c
//
// 以下のクレートを使用。
// - `text_io`

use std::{cmp, panic, process};
use text_io::read;

fn main() {
    panic::set_hook(Box::new(|_| {
        // 変なクエリを吐いて`RE`させ、`TLE`を回避
        println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    }));

    // `text_io::read!`。
    // `read!()`は`read!("{}", std::io::stdin().bytes().map(Result::unwrap))`の短縮となる。
    //
    // https://docs.rs/text_io/0.1.8/text_io/macro.read.html
    let n: usize = read!();

    let query = |i: usize| -> _ {
        println!("{}", i);
        let seat: String = read!();
        match &*seat {
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
