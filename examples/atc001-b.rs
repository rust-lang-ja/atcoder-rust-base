use petgraph::unionfind::UnionFind;

#[proconio::fastout]
fn main() {
    // use std::io::{self, Read as _};
    //
    // let mut input = "".to_owned();
    // io::stdin().read_to_string(&mut input).unwrap();
    // let mut input = input.split_whitespace();
    // macro_rules! read {
    //     ([$t:tt; $n:expr]) => {
    //         (0..$n).map(|_| read!($t)).collect::<Vec<_>>()
    //     };
    //     (($($t:tt),+)) => {
    //         ($(read!($t)),*)
    //     };
    //     (_1based) => {
    //         read!(usize) - 1
    //     };
    //     (_bytes) => {
    //         read!(String).into_bytes()
    //     };
    //     ($ty:ty) => {
    //         input.next().unwrap().parse::<$ty>().unwrap()
    //     };
    // }
    //
    // let (n, q) = read!((usize, usize));
    // let pabs = read!([(u8, usize, usize); q]);

    use proconio::input;

    input! {
        n: usize,
        q: usize,
        pabs: [(u8, usize, usize); q],
    }

    let mut uf = UnionFind::new(n);
    for (p, a, b) in pabs {
        if p == 1 {
            let same = uf.find(a) == uf.find(b);
            println!("{}", if same { "Yes" } else { "No" });
        } else {
            uf.union(a, b);
        }
    }
}
