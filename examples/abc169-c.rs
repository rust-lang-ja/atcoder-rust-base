// https://atcoder.jp/contests/abc169/tasks/abc169_c
//
// 以下のクレートを使用。
// - `num`
//     - `num-rational`
// - `proconio`

use num::rational::Ratio;
use proconio::input;

// 有理数型である[`num_rational::Ratio<_>`]を使う。
//
// `Ratio<T>`に対する`FromStr`は`"Tの形式"`または`"Tの形式/Tの形式"`を受け付ける。
// よって入力をパースするときはAはそのまま、
// Bは小数点下が2桁で固定なので`(_.replace('.', "") + "/100").parse().unwrap()`とすれば良い。
//
// そして[`.to_integer()`]で「0方向に丸めた整数」が得られるので`(_ * _).to_integer()`を答えとすれば良い。
//
// [`num_rational::Ratio<_>`]: https://docs.rs/num-rational/0.2.4/num_rational/struct.Ratio.html
// [`.to_integer()`]: https://docs.rs/num-rational/0.2.4/num_rational/struct.Ratio.html#method.to_integer

fn main() {
    // [`proconio::input!`]で入力を読む。
    //
    // [`proconio::input!`]: https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    input! {
        a: Ratio<u64>,
        b: String,
    }

    let b = (b.replace('.', "") + "/100").parse::<Ratio<_>>().unwrap();
    println!("{}", (a * b).to_integer());
}
