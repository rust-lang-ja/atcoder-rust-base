// https://atcoder.jp/contests/abc054/tasks/abc054_c

use petgraph::csr::Csr;
use petgraph::Undirected;

use std::io::{self, Read as _};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read {
        ([$tt:tt; $n:expr]) => {
            (0..$n).map(|_| read!($tt)).collect::<Vec<_>>()
        };
        (($($tt:tt),+)) => {
            ($(read!($tt)),*)
        };
        (_1based) => {
            read!(usize) - 1
        };
        ($ty:ty) => {
            input.next().unwrap().parse::<$ty>().unwrap()
        };
    }

    let (n, m) = read!((usize, usize));
    let abs = read!([(_1based, _1based); m]);

    let mut graph = Csr::<(), (), Undirected, usize>::with_nodes(n);
    for (a, b) in abs {
        graph.add_edge(a, b, ());
    }
    let mut ans = 0;
    let mut nodes = (0..n).collect::<Vec<_>>();
    permutohedron::heap_recursive(&mut nodes, |nodes| {
        if nodes[0] == 0 && nodes.windows(2).all(|w| graph.contains_edge(w[0], w[1])) {
            ans += 1;
        }
    });
    println!("{}", ans);
}
