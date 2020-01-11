// https://atcoder.jp/contests/abc120/tasks/abc120_d

use union_find::{QuickFindUf, UnionBySize, UnionFind as _};

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
        (_1based) => {
            read!(usize) - 1
        };
        ($ty:ty) => {
            input.next().unwrap().parse::<$ty>().unwrap()
        };
    }

    let (n, m) = read!((usize, usize));
    let abs = read!([(_1based, _1based); m]);

    let max = n * (n - 1) / 2;
    let mut uf = QuickFindUf::<UnionBySize>::new(n);
    let mut ans_rev = vec![max];

    ans_rev.extend(abs.into_iter().rev().scan(max, |cur, (a, b)| {
        let p = uf.get(a).size() * uf.get(b).size();
        if uf.union(a, b) {
            *cur -= p;
        }
        Some(*cur)
    }));
    assert_eq!(ans_rev.pop(), Some(0));

    buf_print(|stdout| {
        macro_rules! println { ($($tt:tt)*) => { writeln!(stdout, $($tt)*).unwrap() }; }

        for x in ans_rev.into_iter().rev() {
            println!("{}", x);
        }
    });
}

fn buf_print(f: impl FnOnce(&mut BufWriter<StdoutLock>)) {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    f(&mut stdout);
    stdout.flush().unwrap();
}
