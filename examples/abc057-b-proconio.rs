// https://atcoder.jp/contests/abc057/tasks/abc057_b

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abs: [(i64, i64); n],
        cds: [(i64, i64); m],
    }

    for (a, b) in abs {
        let j = (0..m)
            .min_by_key(|&j| {
                let (c, d) = cds[j];
                (a - c).abs() + (b - d).abs()
            })
            .unwrap();
        println!("{}", j + 1);
    }
}
