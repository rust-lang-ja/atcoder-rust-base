// https://atcoder.jp/contests/tokiomarine2020/tasks/tokiomarine2020_a
//
// 以下のクレートを使用。
// - `itertools`
// - `itertools-num`
// - `proconio`

use itertools::Itertools as _;
use itertools_num::ItertoolsNum as _;
use proconio::input;
use std::cmp;

fn main() {
    // [`proconio::input!`]で入力を読む。
    //
    // [`proconio::input!`]: https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        n: usize,
        k: usize,
        mut r#as: [usize; n],
    }

    for _ in 0..k {
        // NもAも`usize`で持っておけば、numeric castは一箇所で済む。

        let mut imos = vec![0; n + 1];

        for (i, &a) in r#as.iter().enumerate() {
            let l = i.saturating_sub(a);
            let r = cmp::min(i + a + 1, n);
            imos[l] += 1;
            imos[r] -= 1;
        }

        // [`itertools_num::ItertoolsNum::cumsom`]を使って`imos`を復元する。
        //
        // [`itertools_num::ItertoolsNum::cumsom`]:  https://docs.rs/itertools-num/0.1.3/itertools_num/trait.ItertoolsNum.html#method.cumsum
        r#as = imos[..n].iter().map(|&x| x as usize).cumsum().collect();

        if r#as.iter().all(|&a| a == n) {
            break;
        }
    }

    // [`itertools::Itertools::format`]でスペース区切りにしたものを`println!`する。
    //
    // 注意としてこのメソッドの返り値はイテレータを`RefCell<Option<_>>`の形で保持していており、二度displayしようとするとpanicする。
    //
    // [`itertools::Itertools::format`]: https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html#method.format
    println!("{}", r#as.into_iter().format(" "));
}
