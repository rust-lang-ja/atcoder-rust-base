// https://atcoder.jp/contests/abc121/tasks/abc121_b

use whiteread::Reader;

fn main() {
    let mut rdr = Reader::from_stdin_naive();

    let (n, _, c) = rdr.p::<(usize, usize, i32)>();
    let b = rdr.line::<Vec<i32>>().unwrap();
    let a = (0..n)
        .map(|_| rdr.line().unwrap())
        .collect::<Vec<Vec<i32>>>();

    let ans = a
        .into_iter()
        .filter(|a| a.iter().zip(&b).map(|(a, b)| a * b).sum::<i32>() + c > 0)
        .count();
    println!("{}", ans);
}
