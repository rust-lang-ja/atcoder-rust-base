// https://atcoder.jp/contests/apc001/tasks/apc001_c
//
// 以下のクレートを使用。
// - `whiteread`

use std::{cmp, panic, process};
use whiteread::Reader;

fn main() {
    panic::set_hook(Box::new(|_| {
        // 変なクエリを吐いて`RE`させ、`TLE`を回避
        println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    }));

    // `whiteread::Reader`
    //
    // https://docs.rs/whiteread/0.5.0/whiteread/reader/struct.Reader.html
    let mut rdr = Reader::from_stdin_naive();

    // `whiteread::Reader::line`で行ごとに値を読める。特にこのようなインタラクティブ問題に適している。
    //
    // https://docs.rs/whiteread/0.5.0/whiteread/reader/struct.Reader.html#method.line
    let n = rdr.line::<usize>().unwrap();

    let mut query = |i: usize| -> _ {
        println!("{}", i);
        match &*rdr.line::<String>().unwrap() {
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
