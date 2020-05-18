// https://atcoder.jp/contests/apc001/tasks/apc001_c
//
// `std` only

use std::{cmp, io, panic, process};

macro_rules! read(($ty:ty) => ({
    let mut input = "".to_owned();
    io::stdin().read_line(&mut input).unwrap();
    input.trim_end().parse::<$ty>().unwrap()
}));

fn main() {
    panic::set_hook(Box::new(|_| {
        // 変なクエリを吐いて`RE`させ、`TLE`を回避
        println!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    }));

    let n = read!(usize);

    let query = |i: usize| -> _ {
        println!("{}", i);
        match &*read!(String) {
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
