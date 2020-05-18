// https://atcoder.jp/contests/abc154/tasks/abc154_e
//
// 以下のクレートを使用。
//
// - `num`
// - `proconio`

use proconio::input;
use proconio::marker::Bytes;

fn main() {
    // http://drken1215.hatenablog.com/entry/2020/02/09/225300

    // `proconio::input!`。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        n: Bytes,
        k: usize,
    }

    let n = n.into_iter().map(|d| (d - b'0').into()).collect();

    println!("{}", St { n }.solve(0, k, false));

    struct St {
        n: Vec<usize>,
    }

    impl St {
        fn solve(&self, i: usize, j: usize, p: bool) -> usize {
            let Self { n } = self;

            if j == 0 {
                1
            } else if i == n.len() {
                0
            } else if p {
                // 配列でのDPと違い、ここで打ち切れる。 ここで`num_integer::binomial`が使える。
                //
                // https://docs.rs/num-integer/0.1/num_integer/fn.binomial.html
                num::integer::binomial(n.len() - i, j) * 9usize.pow(j as u32)
            } else if n[i] == 0 {
                self.solve(i + 1, j, false)
            } else {
                self.solve(i + 1, j, true)
                    + self.solve(i + 1, j - 1, true) * (n[i] - 1)
                    + self.solve(i + 1, j - 1, false)
            }
        }
    }
}
