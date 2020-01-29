// https://atcoder.jp/contests/abc057/tasks/abc057_b

use text_io::read;

fn main() {
    let n: usize = read!();
    let m: usize = read!();
    let abs = (0..n)
        .map(|_| (read!(), read!()))
        .collect::<Vec<(i64, i64)>>();
    let cds = (0..m)
        .map(|_| (read!(), read!()))
        .collect::<Vec<(i64, i64)>>();

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
