// https://atcoder.jp/contests/abc142/tasks/abc142_c

use itertools::Itertools as _;
use superslice::Ext2 as _;

use std::io::{self, Read};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read {
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
        ({ Isize1 }) => {
            read!(isize) - 1
        };
    }

    let mut a = read!([{ Isize1 }]);
    a.invert_permutation();
    println!("{}", a.iter().map(|a| a + 1).format(" "));
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
