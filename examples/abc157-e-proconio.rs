// https://atcoder.jp/contests/abc157/tasks/abc157_e
//
// 以下のクレートを使用。
//
// - `alga`
// - `proconio`

use alga::general::{AbstractGroup, AbstractMonoid, Additive, Operator};
use proconio::marker::{Bytes, Usize1};
use proconio::{fastout, input};

use std::marker::PhantomData;
use std::ops::{RangeInclusive, RangeTo, RangeToInclusive};

// `#[proconio::fastout]`で標準出力を高速化する。
//
// https://docs.rs/proconio-derive/0.1.6/proconio_derive/attr.fastout.html
#[fastout]
fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        n: usize,
        mut s: Bytes,
        q: usize,
    }

    let mut bits = vec![Bit::<_, Additive>::new(n); 27];

    macro_rules! bit(($c:expr) => (bits[usize::from($c - b'a')]));

    for (i, c) in s.iter().enumerate() {
        bit!(c).plus(i, &1);
    }

    for _ in 0..q {
        // `proconio::input!`はオリジナルの`input!`とは違い入力を`lazy_static`で保存しているため、2回以上呼ぶことができる。

        input!(kind: u32);
        match kind {
            1 => {
                input!(i: Usize1, c: char);
                let c = c as u8;
                bit!(s[i]).plus(i, &-1);
                bit!(c).plus(i, &1);
                s[i] = c;
            }
            2 => {
                input!(l: Usize1, r: Usize1);
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
