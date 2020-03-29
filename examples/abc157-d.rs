// https://atcoder.jp/contests/abc157/tasks/abc157_d
//
// 以下のクレートを使用。
//
// - `itertools`
// - `petgraph`
// - `proconio`

use itertools::Itertools as _;
use petgraph::graphmap::UnGraphMap;
use petgraph::unionfind::UnionFind;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        n: usize,
        m: usize,
        k: usize,
        abs: [(Usize1, Usize1); m],
        cds: [(Usize1, Usize1); k],
    }

    // `petgraph::unionfind::UnionFind<_>`を使用。
    // `UnionFind`は各集合のサイズは覚えてないので`vec![_; n]`で管理する。
    //
    // https://docs.rs/petgraph/0.5/petgraph/unionfind/struct.UnionFind.html
    let mut ff = UnionFind::new(n);
    let mut ff_sizes = vec![1; n];
    for &(a, b) in &abs {
        let size_sum = ff_sizes[ff.find(a)] + ff_sizes[ff.find(b)];
        if ff.union(a, b) {
            ff_sizes[ff.find(a)] = size_sum;
        }
    }

    // `UnionFind`の`rank`はprivateな為面倒。ここではこのような方法で数える。
    // この際`HashSet`のかわりに`petgraph::graphmap::GraphMap<..>`を使う。
    //
    // https://docs.rs/petgraph/0.5/petgraph/graphmap/struct.GraphMap.html
    let mut ans = (0..n).map(|i| ff_sizes[ff.find(i)] - 1).collect::<Vec<_>>();
    let mut already_decr = UnGraphMap::with_capacity(n, abs.len());
    for (a, b) in abs {
        ans[a] -= 1;
        ans[b] -= 1;
        already_decr.add_edge(a, b, ());
    }
    for (c, d) in cds {
        if ff.equiv(c, d) && !already_decr.contains_edge(c, d) {
            ans[c] -= 1;
            ans[d] -= 1;
        }
    }

    println!("{}", ans.iter().format(" "));
}
