// https://atcoder.jp/contests/judge-update-202004/tasks/judge_update_202004_c
//
// 以下のクレートを使用。
// - `itertools`
// - `itertools-num`
// - `proconio`

use itertools::{iproduct, Itertools as _};
use itertools_num::ItertoolsNum as _;
use proconio::input;
use std::iter;

fn main() {
    // `proconio::input!`で入力を読む。
    //
    // https://docs.rs/proconio/0.3/proconio/macro.input.html
    input! {
        r#as: [usize; 3],
    }

    // `itertools::Itertools::permutations`で順列を列挙。
    // 一つ一つ`Vec<_>`を作るので気になるなら`superslice`/`permutohedron`の`next_permutation`か`permutohedron`の`heap_recursive`を。
    //
    // https://docs.rs/itertools/0.9/itertools/trait.Itertools.html#method.permutations
    // https://docs.rs/superslice/1/superslice/trait.Ext.html#tymethod.next_permutation
    // https://docs.rs/permutohedron/0.2/permutohedron/trait.LexicalPermutation.html#tymethod.next_permutation
    // https://docs.rs/permutohedron/0.2/permutohedron/fn.heap_recursive.html
    let ans = (0..r#as.iter().sum())
        .permutations(r#as.iter().sum())
        .filter(|perm| {
            // `itertools_num::ItertoolsNum::cumsum`で累積和のイテレータが得られる。
            // そしてその"windows"を`itertools::Itertools::tuple_windows`でイテレータからそのまま得る。
            // 今回場合`perm`を3分割するだけだが..
            //
            // https://docs.rs/itertools-num/0.1/itertools_num/trait.ItertoolsNum.html#method.cumsum
            // https://docs.rs/itertools/0.9/itertools/trait.Itertools.html#method.tuple_combinations
            let x = iter::once(&0)
                .chain(&r#as)
                .cumsum()
                .tuple_windows()
                .map(|(c1, c2)| &perm[c1..c2])
                .collect::<Vec<_>>();

            // `for i in 0..3 { for j in 0..3 { .. } }`の代わりに`itertools::iproduct!`を使う。
            //
            // https://docs.rs/itertools/0.9/itertools/macro.iproduct.html

            let horz = iproduct!(0..3, 0..3)
                .filter(|&(i, j)| i > 0 && j < r#as[i])
                .all(|(i, j)| x[i][j] > x[i - 1][j]);

            let vert = iproduct!(0..3, 0..3)
                .filter(|&(i, j)| j > 0 && j < r#as[i])
                .all(|(i, j)| x[i][j] > x[i][j - 1]);

            horz && vert
        })
        .count();
    println!("{}", ans);
}
