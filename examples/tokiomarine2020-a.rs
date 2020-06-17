// https://atcoder.jp/contests/tokiomarine2020/tasks/tokiomarine2020_a
//
// 以下のクレートを使用。
// - `ascii`
// - `proconio`

use ascii::AsciiString;
use proconio::input;

fn main() {
    // Sを[`ascii::AsciiString`]として、[`proconio::input!`]で入力を読む。
    //
    // [`ascii::AsciiString`]: https://docs.rs/ascii/1.0.0/ascii/struct.AsciiString.html
    // [`proconio::input!`]:   https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        s: AsciiString,
    }

    // [`AsciiStr`] / `AsciiString`は
    //
    // 1. [`usize`の範囲でアクセス可能]であり**かつ**
    // 2. そのまま[`Display`]可能
    //
    // であることから、このような問題で変換を挟まずに簡潔に書くことができる。
    //
    // `ascii`クレートを使わずに行うなら、
    //
    // 1. Sを`String`として読み、`&s[..3]`としてスライスを取得する
    // 2. Sを`String`として読み、`s.chars().take(3).collect::<String>()`
    // 3. Sを[`proconio::marker::Bytes`]経由で`Vec<u8>`として読み(あるいは`String`から`.into_bytes()`する)、`std::str::from_utf8(&s[..3]).unwrap()`
    //
    // の2つの方法がある。
    //
    // [`AsciiStr`]:                  https://docs.rs/ascii/1.0.0/ascii/struct.AsciiStr.html
    // [`usize`の範囲でアクセス可能]: https://docs.rs/ascii/1.0.0/ascii/struct.AsciiStr.html#impl-Index%3CRangeTo%3Cusize%3E%3E
    // [`Display`]:                   https://doc.rust-lang.org/1.42.0/std/fmt/trait.Display.html
    // [`proconio::marker::Bytes`]:   https://docs.rs/proconio/0.3.6/proconio/marker/enum.Bytes.html
    println!("{}", &s[..3]);
}
