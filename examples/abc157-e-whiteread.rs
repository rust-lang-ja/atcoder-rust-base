// https://atcoder.jp/contests/abc157/tasks/abc157_e
//
// 以下のクレートを使用。
//
// - `alga`
// - `proconio`
// - `whiteread`

use alga::general::{AbstractGroup, AbstractMonoid, Additive, Operator};
use proconio::fastout;
use whiteread::Reader;

use std::marker::PhantomData;
use std::ops::{RangeInclusive, RangeTo, RangeToInclusive};

// `#[proconio::fastout]`で標準出力を高速化する。
//
// https://docs.rs/proconio-derive/0.1.6/proconio_derive/attr.fastout.html
#[fastout]
fn main() {
    // `whiteread::Reader`で入力を読む。
    //
    // https://docs.rs/whiteread/0.5.0/whiteread/reader/struct.Reader.html
    let mut rdr = Reader::from_stdin_naive();

    let n = rdr.p::<usize>();
    let mut s = rdr.p::<String>().into_bytes();

    let mut bits = vec![Bit::<_, Additive>::new(n); 27];

    macro_rules! bit(($c:expr) => (bits[usize::from($c - b'a')]));

    for (i, c) in s.iter().enumerate() {
        bit!(c).plus(i, &1);
    }

    for _ in 0..rdr.p() {
        match rdr.p() {
            1 => {
                let (i, c) = (rdr.p::<usize>() - 1, rdr.p::<char>() as u8);
                bit!(s[i]).plus(i, &-1);
                bit!(c).plus(i, &1);
                s[i] = c;
            }
            2 => {
                let (l, r) = (rdr.p::<usize>() - 1, rdr.p::<usize>() - 1);
                let ans = bits.iter().filter(|bits| bits.query(l..=r) > 0).count();
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
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
