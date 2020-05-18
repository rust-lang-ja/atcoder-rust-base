// https://atcoder.jp/contests/abc157/tasks/abc157_e
//
// 以下のクレートを使用。
//
// - `alga`

use alga::general::{AbstractGroup, AbstractMonoid, Additive, Operator};

use std::io::{self, BufWriter, Read as _, StdoutLock, Write as _};
use std::marker::PhantomData;
use std::ops::{RangeInclusive, RangeTo, RangeToInclusive};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        //([$tt:tt]) => (read!([$tt; read!(usize)]));
        //([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
        ({ Bytes }) => (read!(String).into_bytes());
        ({ Byte }) => (read!(char) as u8);
        ({ Usize1 }) => (read!(usize) - 1);
    );

    let (n, mut s) = read!((usize, { Bytes }));

    let mut bits = vec![Bit::<_, Additive>::new(n); 27];

    macro_rules! bit(($c:expr) => (bits[usize::from($c - b'a')]));

    for (i, c) in s.iter().enumerate() {
        bit!(c).plus(i, &1);
    }

    buf_print(|stdout| {
        macro_rules! println(($($tt:tt)*) => (writeln!(stdout, $($tt)*).unwrap()));
        for _ in 0..read!(_) {
            match read!(_) {
                1 => {
                    let (i, c) = read!(({ Usize1 }, { Byte }));
                    bit!(s[i]).plus(i, &-1);
                    bit!(c).plus(i, &1);
                    s[i] = c;
                }
                2 => {
                    let (l, r) = read!(({ Usize1 }, { Usize1 }));
                    let ans = bits.iter().filter(|bits| bits.query(l..=r) > 0).count();
                    println!("{}", ans);
                }
                _ => unreachable!(),
            }
        }
    });
}

// BIT (Binary Indexed Tree)を`alga::general`で抽象化する。
//
// https://docs.rs/alga/0.9/alga/general/index.html

#[derive(Clone, Debug)]
struct Bit<M, O> {
    nodes: Vec<M>,
    phantom: PhantomData<fn() -> O>,
}

impl<M: AbstractMonoid<O>, O: Operator> Bit<M, O> {
    fn new(n: usize) -> Self {
        Self {
            nodes: vec![M::identity(); n],
            phantom: PhantomData,
        }
    }

    fn query<R: BitIndex<M, O>>(&self, range: R) -> M {
        range.query(&self.nodes)
    }

    fn plus(&mut self, i: usize, x: &M) {
        let mut i_1based = i + 1;
        while i_1based <= self.nodes.len() {
            self.nodes[i_1based - 1] = self.nodes[i_1based - 1].operate(x);
            i_1based += 1 << i_1based.trailing_zeros();
        }
    }
}

trait BitIndex<M: AbstractMonoid<O>, O: Operator> {
    fn query(&self, nodes: &[M]) -> M;
}

impl<M: AbstractMonoid<O>, O: Operator> BitIndex<M, O> for RangeTo<usize> {
    fn query(&self, nodes: &[M]) -> M {
        #[allow(clippy::range_minus_one)]
        match self.end {
            0 => M::identity(),
            end => (..=end - 1).query(nodes),
        }
    }
}

impl<M: AbstractMonoid<O>, O: Operator> BitIndex<M, O> for RangeToInclusive<usize> {
    fn query(&self, nodes: &[M]) -> M {
        let mut acc = M::identity();
        let mut i_1based = self.end + 1;
        while i_1based > 0 {
            acc = acc.operate(&nodes[i_1based - 1]);
            i_1based -= 1 << i_1based.trailing_zeros();
        }
        acc
    }
}

impl<M: AbstractGroup<O>, O: Operator> BitIndex<M, O> for RangeInclusive<usize> {
    fn query(&self, nodes: &[M]) -> M {
        let l = (..*self.start()).query(nodes);
        let r = (..=*self.end()).query(nodes);
        r.operate(&l.two_sided_inverse())
    }
}

fn buf_print<F: FnMut(&mut BufWriter<StdoutLock<'_>>)>(mut f: F) {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    f(&mut stdout);
    stdout.flush().unwrap();
}
