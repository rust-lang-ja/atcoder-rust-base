// https://atcoder.jp/contests/practice/tasks/practice_2
//
// 以下のクレートを使用。
//
// - `itertools`
// - `maplit`
// - `proconio`

use maplit::hashset;
use proconio::{input, source::line::LineSource};
use std::{
    io::{self, BufReader},
    str,
};

fn main() {
    // `proconio::input!`はrelease modeではデフォルトでは`OnceSource`を使ってしまうので、
    // `input! { from: .., }`で`LineSource`を指定する必要がある。
    // `Source`を指定するときはこのように部分適用したマクロを用意しておくと楽。
    //
    // https://docs.rs/proconio/0.3.6/proconio/macro.input.html
    // https://docs.rs/proconio/0.3.6/proconio/source/once/struct.OnceSource.html
    // https://docs.rs/proconio/0.3.6/proconio/source/line/struct.LineSource.html
    // https://docs.rs/proconio/0.3.6/proconio/source/trait.Source.html
    let mut stdin = LineSource::new(BufReader::new(io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input!(n: u32, _: u32);

    let query = |l: u8, r: u8| -> _ {
        println!("? {} {}", l as char, r as char);
        input!(c: char);
        c == '<'
    };

    let ans = match n {
        26 => on_26(query),
        5 => on_5(query),
        _ => unreachable!(),
    };
    println!("! {}", str::from_utf8(&ans).unwrap());
}

fn on_26(mut query: impl FnMut(u8, u8) -> bool) -> Vec<u8> {
    (b'B'..=b'Z').fold(vec![b'A'], |balls, ball| insort(balls, ball, &mut query))
}

#[allow(clippy::many_single_char_names)]
fn on_5(mut query: impl FnMut(u8, u8) -> bool) -> Vec<u8> {
    let (r, s, t, u) = {
        let (q1, q2) = (query(b'A', b'B'), query(b'C', b'D'));
        let (light1, heavy1) = if q1 { (b'A', b'B') } else { (b'B', b'A') };
        let (light2, heavy2) = if q2 { (b'C', b'D') } else { (b'D', b'C') };
        let q3 = query(light1, light2);
        if q3 {
            (light1, heavy1, light2, heavy2)
        } else {
            (light2, heavy2, light1, heavy1)
        }
    };

    let v = (&hashset!(b'A', b'B', b'C', b'D', b'E') - &hashset!(r, s, t, u))
        .into_iter()
        .next()
        .unwrap();

    let q4 = query(t, v);
    if q4 {
        let q5 = query(u, v);
        let (min_uv, max_uv) = if q5 { (u, v) } else { (v, u) };
        itertools::chain(vec![r], insort(vec![t, min_uv, max_uv], s, &mut query)).collect()
    } else {
        let q5 = query(r, v);
        if q5 {
            itertools::chain(vec![r], insort(vec![v, t, u], s, &mut query)).collect()
        } else {
            itertools::chain(vec![v, r], insort(vec![t, u], s, &mut query)).collect()
        }
    }
}

fn insort(mut balls: Vec<u8>, ball: u8, mut query: impl FnMut(u8, u8) -> bool) -> Vec<u8> {
    let (mut min, mut max) = (0, balls.len());
    while min < max {
        let mid = (min + max) / 2;
        if query(balls[mid], ball) {
            min = mid + 1;
        } else {
            max = mid
        };
    }
    balls.insert(min, ball);
    balls
}

#[cfg(test)]
mod tests {
    use itertools::Itertools as _;

    use std::str;

    #[test]
    fn on_5() {
        for balls in (b'A'..=b'E').permutations(5) {
            let mut queries = 0;
            let ans = super::on_5(|l, r| {
                queries += 1;
                let wl = balls.iter().position(|&b| b == l).unwrap();
                let wr = balls.iter().position(|&b| b == r).unwrap();
                wl < wr
            });
            let ans = str::from_utf8(&ans).unwrap();
            let balls = str::from_utf8(&balls).unwrap();
            assert_eq!(ans, balls);
            assert!(queries <= 7);
        }
    }
}
