// https://atcoder.jp/contests/abc168/tasks/abc168_b
//
// 以下のクレートを使用。
// - `ascii`
// - `proconio`

use ascii::{AsciiStr, AsciiString};
use proconio::input;

fn main() {
    // `str`/`String`や`[u8]`/`Vec<u8>`のかわりに`ascii::AsciiStr`/`ascii/AsciiString`を使うことができる。
    //
    // https://docs.rs/ascii/1.0.0/ascii/struct.AsciiStr.html
    // https://docs.rs/ascii/1.0.0/ascii/struct.AsciiString.html

    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        k: usize,
        mut s: AsciiString,
    }

    if s.len() > k {
        s.truncate(k);
        s += AsciiStr::from_ascii(b"...").unwrap();
    }

    println!("{}", s);
}

// 参考: Sを`Vec<u8>`で取った場合
const _: fn() = || {
    use proconio::marker::Bytes;

    input! {
        k: usize,
        mut s: Bytes,
    }

    if s.len() > k {
        s.truncate(k);
        s.extend_from_slice(b"...");
    }

    println!("{}", String::from_utf8(s).unwrap());
};
