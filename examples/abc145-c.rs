// https://atcoder.jp/contests/abc145/tasks/abc145_c
//
// 以下のクレートを使用。
//
// - `itertools`
// - `nalgebra`
// - `proconio`

use itertools::Itertools as _;
use nalgebra::{Point2, Scalar};
use proconio::input;
use proconio::source::{Readable, Source};

use std::convert::Infallible;
use std::io::BufRead;
use std::marker::PhantomData;

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        n: usize,
        points: [ReadPoint2<f64>; n],
    }

    // nC2通りの組み合わせを列挙するのに`<_ as itertools::Itertools>::tuple_combinations`を用いる。
    // また各点同士の距離√((x_i - x_j)^2 + (y_i - y_j)^2)を求めるのに`nalgebra::distance`を使う。
    //
    // https://docs.rs/itertools/0.8/itertools/trait.Itertools.html#method.tuple_combinations
    // https://docs.rs/nalgebra/0.19/nalgebra/fn.distance.html
    let ans = 2.0 / (n as f64)
        * points
            .into_iter()
            .tuple_combinations()
            .map(|(p1, p2)| nalgebra::distance(&p1, &p2))
            .sum::<f64>();
    println!("{}", ans);
}

// `proconio::source::Readable`を実装することで`Usize1`のようなマーカー型を作ることができる。
//
// https://docs.rs/proconio/0.3.6/proconio/source/trait.Readable.html

struct ReadPoint2<T>(Infallible, PhantomData<fn() -> T>);

impl<N: Readable<Output = N> + Scalar> Readable for ReadPoint2<N> {
    type Output = Point2<N>;

    fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Point2<N> {
        Point2::new(N::read(source), N::read(source))
    }
}
