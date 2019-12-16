// https://atcoder.jp/contests/abc054/tasks/abc054_c

use petgraph::csr::Csr;
use petgraph::Undirected;

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
    // let (n, m) = read!((usize, usize));
    // let mut abs = read!([(_1based, _1based); m]);

    use proconio::input;
    use proconio::marker::Usize1;

    input! {
        n: usize,
        m: usize,
        mut abs: [(Usize1, Usize1); m],
    }

    abs.sort();
    let mut g = Csr::<(), (), Undirected, usize>::with_nodes(n);
    for (a, b) in abs {
        g.add_edge(a, b, ());
    }
    let mut ans = 0;
    let mut es = (0..n).collect::<Vec<_>>();
    permutohedron::heap_recursive(&mut es, |es| {
        if es[0] == 0 && es.windows(2).all(|w| g.contains_edge(w[0], w[1])) {
            ans += 1;
        }
    });
    println!("{}", ans);
}
