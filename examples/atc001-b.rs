// https://atcoder.jp/contests/atc001/tasks/unionfind_a

use petgraph::unionfind::UnionFind;

use std::io::{self, BufWriter, Read as _, StdoutLock, Write as _};

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
        ($ty:ty) => {
            input.next().unwrap().parse::<$ty>().unwrap()
        };
    }

    let (n, q) = read!((usize, usize));
    let pabs = read!([(u8, usize, usize); q]);

    let mut uf = UnionFind::new(n);
    buf_print(|stdout| {
        macro_rules! println { ($($tt:tt)*) => { writeln!(stdout, $($tt)*).unwrap() }; }

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

fn buf_print(f: impl FnOnce(&mut BufWriter<StdoutLock>)) {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    f(&mut stdout);
    stdout.flush().unwrap();
}
