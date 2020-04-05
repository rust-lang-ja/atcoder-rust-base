// https://atcoder.jp/contests/judge-update-202004/tasks/judge_update_202004_a
//
// 以下のクレートを使用。
// - `num`
//     - `num-traits`
// - `proconio`

use proconio::input;

fn main() {
    // `proconio::input!`で入力を読む。
    //
    // https://docs.rs/proconio/0.3/proconio/macro.input.html
    input! {
        s: i32,
        l: i32,
        r: i32,
    }

    // `num_traits::clamp`で解く。
    // 定義は`clamp(s, l, r) == if s < l { l } else if s > r { r } else { s }`。
    // `PartialOrd`までしか要求しないので`f64`に対しても使える。
    //
    // https://docs.rs/num-traits/0.2/num_traits/fn.clamp.html
    println!("{}", num::clamp(s, l, r));
}
