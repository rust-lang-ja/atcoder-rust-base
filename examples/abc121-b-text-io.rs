// https://atcoder.jp/contests/abc121/tasks/abc121_b

use text_io::read;

#[allow(clippy::many_single_char_names)]
fn main() {
    let (n, m, c): (usize, usize, i32) = (read!(), read!(), read!());
    let b = (0..m).map(|_| read!()).collect::<Vec<i32>>();
    let a = (0..n)
        .map(|_| (0..m).map(|_| read!()).collect())
        .collect::<Vec<Vec<i32>>>();

    let ans = a
        .into_iter()
        .filter(|a| a.iter().zip(&b).map(|(a, b)| a * b).sum::<i32>() + c > 0)
        .count();
    println!("{}", ans);
}
