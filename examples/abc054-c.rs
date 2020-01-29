// https://atcoder.jp/contests/abc054/tasks/abc054_c

use itertools::Itertools as _;
use petgraph::graph::UnGraph;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        abs: [(Usize1, Usize1)],
    }

    let graph = UnGraph::<(), (), usize>::from_edges(abs);
    let ans = graph
        .node_indices()
        .permutations(n)
        .filter(|p| p[0].index() == 0 && p.windows(2).all(|w| graph.contains_edge(w[0], w[1])))
        .count();
    println!("{}", ans);
}
