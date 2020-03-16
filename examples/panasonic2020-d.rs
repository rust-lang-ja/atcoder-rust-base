// https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_d
//
// 以下のクレートを使用。
//
// - `proconio`
// - `smallvec`

use proconio::{fastout, input};
use smallvec::{smallvec, Array, SmallVec};

use std::collections::VecDeque;
use std::{cmp, str};

// `#[proconio::fastout]`で標準出力を高速化する。
//
// https://docs.rs/proconio-derive/0.1.6/proconio_derive/attr.fastout.html
#[fastout]
fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        n: usize,
    }

    // **高々**長さ10の文字列を`SmallVec<[u8; 10]>`で表わす。
    //
    // https://docs.rs/smallvec/1/smallvec/struct.SmallVec.html
    let mut queue = VecDeque::<(SmallVec<[_; 10]>, _)>::with_capacity(1 << 16);
    queue.push_back((smallvec![b'a'], b'a'));

    while let Some((s, max)) = queue.pop_front() {
        if s.len() == n {
            println!("{}", str::from_utf8(&s).unwrap());
        } else {
            for c in b'a'..=max + 1 {
                queue.push_back((concat(s.clone(), c), cmp::max(c, max)));
            }
        }
    }
}

fn concat<A: Array>(mut xs: SmallVec<A>, x: A::Item) -> SmallVec<A> {
    xs.push(x);
    xs
}
