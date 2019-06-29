// -*- coding:utf-8-unix -*-

type UnitResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> UnitResult {
    run_proconio();
    run_ascii()?;
    run_bitset_fixed();
    run_superslice();
    run_itertools();
    run_rand_family()?;
    run_sfmt()?;
    Ok(())
}

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

// proconio, proconio-derive
#[proconio_derive::fastout]
fn run_proconio() {
    use proconio::input;
    use proconio::source::auto::AutoSource;

    let source = AutoSource::from(
        r#"2
3 1 2
6 1 1
"#,
    );

    input! {
        from source,
        n: usize,
        mut plan: [(i32, i32, i32); n],  // Vec<(i32, i32, i32)>
    }
    plan.insert(0, (0, 0, 0));
    let yes = plan.windows(2).all(|w| {
        let (t0, x0, y0) = w[0];
        let (t1, x1, y1) = w[1];
        let time = t1 - t0;
        let dist = (x1 - x0).abs() + (y1 - y0).abs();
        dist <= time && time % 2 == dist % 2
    });
    println!("{}", if yes { "Yes" } else { "No" });
    assert!(yes);
}

#[test]
fn test_proconio() {
    run_proconio();
}

// ordered-float

// modtype

// ascii
fn run_ascii() -> UnitResult {
    use ascii::AsciiString;
    // use ascii::{AsciiChar, AsciiStr, AsciiString};
    use std::str::FromStr;

    let s = AsciiString::from_str("2019-07-01")?;
    let mut ix = s.as_str().match_indices("-");
    let (i0, _) = ix.next().ok_or_else(|| "got none")?;
    let (i1, _) = ix.next().ok_or_else(|| "got none")?;

    assert_eq!(s[..i0].as_str(),       "2019");
    assert_eq!(s[i0 + 1..i1].as_str(), "07");
    assert_eq!(s[i1 + 1..].as_str(),   "01");

    // split is not available in ascii 0.9.1
    // https://github.com/tomprogrammer/rust-ascii/issues/62
    //
    // let ymd = s.split(AsciiChar::Minus)
    //     .map(AsciiStr::as_str)
    //     .collect::<Vec<_>>();
    // assert_eq!(ymd, ["2019", "07", "01"]);
    Ok(())
}

#[test]
fn test_ascii() -> UnitResult {
    run_ascii()
}

// bitset-fixed
// This code was taken from an example on: https://crates.io/crates/bitset-fixed
fn run_bitset_fixed() {
    use bitset_fixed::BitSet;
    use rand::distributions::Uniform;
    use rand::prelude::*;

    let mut rng = StdRng::seed_from_u64(114514);
    let dist = Uniform::from(0..2000);

    let n = rng
        .sample_iter::<usize, _>(&dist)
        .take(25)
        .collect::<Vec<_>>();
    let sum = n.iter().sum::<usize>();

    let mut bitset = BitSet::new(sum + 1);
    bitset.set(0, true);

    for &x in &n {
        bitset |= &(&bitset << x);
    }

    let ans = ((sum + 1) / 2..).find(|&i| bitset[i]).unwrap();

    println!("N = {:?}\nAnswer = {}", n, ans);
    assert_eq!(ans, 14675);
}

#[test]
fn test_bitset_fixed() {
    run_bitset_fixed()
}

// permutohedron

// superslice
fn run_superslice() {
    use superslice::Ext;

    let b = [1, 3];
    assert_eq!(b.lower_bound(&1), 0);
    assert_eq!(b.upper_bound(&1), 1);
    assert_eq!(b.equal_range(&3), 1..2);
}

#[test]
fn test_superslice() {
    run_superslice();
}

// itertools
// This code was taken from an example on: https://docs.rs/itertools/0.8.0/itertools/
fn run_itertools() {
    use itertools::Itertools;

    let it = (1..=3).interleave(vec![-1, -2, -3]);
    itertools::assert_equal(it, vec![1, -1, 2, -2, 3, -3]);
}

#[test]
fn test_itertools() {
    run_itertools();
}

// rustc-hash

// hashbrown

// smallvec
// arrayvec

// im
// im-rc

// num

// rand, rand_chacha, rang_pcg
fn run_rand_family() -> UnitResult {
    use rand::prelude::*;
    use rand_chacha::ChaChaRng;
    use rand_pcg::Pcg64Mcg;

    let mut rng = SmallRng::from_rng(thread_rng())?;
    let mean = calc_mean(&mut rng);
    println!("SmallRng: mean = {:.4}", mean);
    assert_eq!((mean * 10.0).round() as u32, 5);

    let mut rng = Pcg64Mcg::from_rng(thread_rng())?;
    let mean = calc_mean(&mut rng);
    println!("ChaChaRng: mean = {:.4}", mean);
    assert_eq!((mean * 10.0).round() as u32, 5);

    let mut rng = ChaChaRng::from_rng(thread_rng())?;
    let mean = calc_mean(&mut rng);
    println!("Pcg64Mcg: mean = {:.4}", mean);
    assert_eq!((mean * 10.0).round() as u32, 5);

    Ok(())
}

#[test]
fn test_rand_family() -> UnitResult {
    run_rand_family()
}

// sfmt
fn run_sfmt() -> UnitResult {
    use rand::prelude::*;
    use sfmt::SFMT;

    let mut rng = SFMT::from_rng(thread_rng())?;
    let mean = calc_mean(&mut rng);
    println!("SFMT: mean = {:.4}", mean);
    assert_eq!((mean * 10.0).round() as u32, 5);
    Ok(())
}

#[test]
fn test_sfmt() -> UnitResult {
    run_sfmt()
}

fn calc_mean(rng: &mut impl rand::Rng) -> f64 {
    const ITERATIONS: usize = 10000;

    // the stardard distribution for f64 generates a random rumber in interval [0, 1)
    rng.sample_iter::<f64, _>(&rand::distributions::Standard)
        .take(ITERATIONS)
        .enumerate()
        // calculate the mean iteratively. https://stackoverflow.com/a/1934266
        .fold(0.0, |mean, (t, x)| mean + (x - mean) / (t + 1) as f64)
}

// regex

// jemallocator
