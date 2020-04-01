// https://atcoder.jp/contests/abc073/tasks/abc073_d

use itertools::Itertools as _;
use num::traits::One;
use petgraph::graph::{IndexType, NodeIndex, UnGraph};
use proconio::input;
use proconio::source::{Readable, Source};

use std::collections::HashMap;
use std::io::BufRead;
use std::marker::PhantomData;
use std::ops::Sub;

fn main() {
    input! {
        _: usize,
        m: usize,
        r: usize,
        rs: [NodeIndex1<u32>; r],
        abcs: [(NodeIndex1<u32>, NodeIndex1<u32>, u32); m],
    }

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

struct NodeIndex1<Ix>(PhantomData<fn() -> Ix>);

impl<Ix: IndexType + Readable<Output = Ix> + One + Sub<Output = Ix>> Readable for NodeIndex1<Ix> {
    type Output = NodeIndex<Ix>;

    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> NodeIndex<Ix> {
        NodeIndex::from(Ix::read(source) - Ix::one())
    }
}
