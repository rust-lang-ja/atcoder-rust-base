// https://atcoder.jp/contests/abc057/tasks/abc057_b

use whiteread::Reader;

fn main() {
    let mut rdr = Reader::from_stdin_naive();

    let (n, m) = rdr.p::<(usize, usize)>();
    let abs = (0..n).map(|_| rdr.p()).collect::<Vec<(i64, i64)>>();
    let cds = (0..m).map(|_| rdr.p()).collect::<Vec<(i64, i64)>>();

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
