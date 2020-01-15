// https://atcoder.jp/contests/abc073/tasks/abc073_d

use itertools::Itertools as _;
use petgraph::graph::{NodeIndex, UnGraph};

use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read {
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
        ({ NodeIndex1 }) => {
            NodeIndex::from(read!(u32) - 1)
        };
    }

    let (_, m, r) = read!((usize, usize, usize));
    let rs = read!([{ NodeIndex1 }; r]);
    let abcs = read!([({ NodeIndex1 }, { NodeIndex1 }, u32); m]);

    let graph = UnGraph::<(), u32>::from_edges(abcs);

    let dijkstra = rs
        .iter()
        .map(|&r| {
            let dijkstra = petgraph::algo::dijkstra(&graph, r, None, |e| *e.weight());
            (r, dijkstra)
        })
        .collect::<HashMap<_, _>>();

    let ans = rs
        .into_iter()
        .permutations(r)
        .map(|rs| rs.windows(2).map(|w| dijkstra[&w[0]][&w[1]]).sum::<u32>())
        .min()
        .unwrap();
    println!("{}", ans);
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
