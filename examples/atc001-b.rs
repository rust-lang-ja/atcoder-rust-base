// https://atcoder.jp/contests/atc001/tasks/unionfind_a

use petgraph::unionfind::UnionFind;

use std::io::{self, BufWriter, Read, StdoutLock, Write as _};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read {
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    }

    let n = read!(usize);
    let pabs = read!([(u8, usize, usize)]);

    let mut uf = UnionFind::new(n);
    buf_print(|stdout| {
        macro_rules! println(($($tt:tt)*) => (writeln!(stdout, $($tt)*).unwrap()));

        for (p, a, b) in pabs {
            if p == 1 {
                let same = uf.find(a) == uf.find(b);
                println!("{}", if same { "Yes" } else { "No" });
            } else {
                uf.union(a, b);
            }
        }
    });
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}

fn buf_print(f: impl FnOnce(&mut BufWriter<StdoutLock>)) {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    f(&mut stdout);
    stdout.flush().unwrap();
}
