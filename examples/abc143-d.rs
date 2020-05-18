// https://atcoder.jp/contests/abc143/tasks/abc143_d
//
// 以下のクレートを使用。
//
// - `itertools`
// - `proconio`
// - `superslice`

use itertools::Itertools as _;
use proconio::input;
use superslice::Ext as _;

fn main() {
    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        mut ls: [u64],
    }

    ls.sort();

    // aとbのnC2通りの組み合わせを`<_ as itertools::Itertools>::tuple_combinations`を使って添字付きで列挙。
    // そしてa, bに対するcの最小値の位置を`<[u64] as superslice::Ext>::upper_bound`で求め、bの位置との差を数え上げたものを答えとする。
    // そのようなcが存在しないとき、`ls[..i].upper_bound(..)`は(C++と同じように)`i`を返すので数える対象は減算により`0`になる。
    //
    // https://docs.rs/itertools/0.8/itertools/trait.Itertools.html#method.tuple_combinations
    // https://docs.rs/superslice/1/superslice/trait.Ext.html#tymethod.upper_bound
    let ans = ls
        .iter()
        .enumerate()
        .tuple_combinations()
        .map(|((i, b), (_, a))| i - ls[..i].upper_bound(&(a - b)))
        .sum::<usize>();
    println!("{}", ans);
}
