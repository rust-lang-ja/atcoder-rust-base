// https://atcoder.jp/contests/abc054/tasks/abc054_c

use itertools::Itertools as _;
use petgraph::graph::UnGraph;

use std::io::{self, Read};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read {
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
        ({ Usize1 }) => {
            read!(usize) - 1
        };
    }

    let n = read!(usize);
    let abs = read!([({ Usize1 }, { Usize1 })]);

    let graph = UnGraph::<(), (), usize>::from_edges(abs);
    let ans = graph
        .node_indices()
        .permutations(n)
        .filter(|p| p[0].index() == 0 && p.windows(2).all(|w| graph.contains_edge(w[0], w[1])))
        .count();
    println!("{}", ans);
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
