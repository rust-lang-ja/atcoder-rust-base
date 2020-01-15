// https://atcoder.jp/contests/abc054/tasks/abc054_c

use petgraph::matrix_graph::UnMatrix;

use std::io::{self, Read};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read {
        (_1based) => {
            read!(usize) - 1
        };
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    }

    let n = read!(usize);
    let abs = read!([(_1based, _1based)]);

    let graph = UnMatrix::<(), (), Option<()>, usize>::from_edges(abs);
    let mut ans = 0;
    let mut nodes = (0..n).map(Into::into).collect::<Vec<_>>();
    permutohedron::heap_recursive(&mut nodes, |nodes| {
        if nodes[0] == 0.into() && nodes.windows(2).all(|w| graph.has_edge(w[0], w[1])) {
            ans += 1;
        }
    });
    println!("{}", ans);
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
