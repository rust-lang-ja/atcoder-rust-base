// https://atcoder.jp/contests/judge-update-202004/tasks/judge_update_202004_d
//
// 以下のクレートを使用。
// - `proconio`
// - `superslice`

use proconio::{fastout, input};
use superslice::Ext as _;

// `#[proconio::fastout]`で標準出力を高速化する。
//
// https://docs.rs/proconio-derive/0.1/proconio_derive/attr.fastout.html
#[fastout]
fn main() {
    // `proconio::input!`で入力を読む。
    //
    // https://docs.rs/proconio/0.3/proconio/macro.input.html
    input! {
        n: usize,
        q: usize,
        r#as: [usize; n],
        ss: [usize; q],
    }

    // `num_integer::gcd`でGCDを得る。
    //
    // https://docs.rs/num-integer/0.1/num_integer/fn.gcd.html
    let gcds = r#as
        .into_iter()
        .scan(0, |gcd, a| {
            *gcd = num::integer::gcd(*gcd, a);
            Some(*gcd)
        })
        .collect::<Vec<_>>();

    for s in ss {
        // "j"を`superslice::Ext::lower_bound_by`で二分探索することで求める。
        //
        // https://docs.rs/superslice/1/superslice/trait.Ext.html#tymethod.lower_bound_by
        let j = gcds.lower_bound_by(|&gcd| 1usize.cmp(&num::integer::gcd(gcd, s)));
        let ans = if j < n {
            j + 1
        } else {
            num::integer::gcd(s, gcds[n - 1])
        };
        println!("{}", ans);
    }
}
