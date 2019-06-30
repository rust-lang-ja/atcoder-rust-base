// -*- coding:utf-8-unix -*-

type UnitResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> UnitResult {
    run_proconio();
    run_modtype()?;
    run_modtype_derive();
    run_ascii()?;
    run_bitset_fixed();
    run_superslice();
    run_itertools();
    run_rand_family()?;
    run_sfmt()?;
    run_regex()?;
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
// these codes were taken from examples at https://github.com/qryxip/modtype/tree/master/examples
fn run_modtype() -> UnitResult {
    use modtype::use_modtype;

    #[use_modtype]
    type F = modtype::u64::F<1_000_000_007u64>;

    let mut a = "13".parse::<F>()?;
    a += F(1_000_000_000);
    assert_eq!(a, F(6));

    Ok(())
}

#[test]
fn test_modtype() -> UnitResult {
    run_modtype()
}

// these codes were taken from examples at https://github.com/qryxip/modtype/blob/master/examples/derive.rs
fn run_modtype_derive() {
    use modtype::{use_modtype, ConstValue};
    use std::marker::PhantomData;

    #[use_modtype]
    type F = F_<17u32>;

    #[derive(
        modtype::new,
        modtype::new_unchecked,
        modtype::get,
        Default,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        modtype::From,
        modtype::Into,
        modtype::Display,
        modtype::Debug,
        modtype::FromStr,
        modtype::Deref,
        modtype::Neg,
        modtype::Add,
        modtype::AddAssign,
        modtype::Sub,
        modtype::SubAssign,
        modtype::Mul,
        modtype::MulAssign,
        modtype::Div,
        modtype::DivAssign,
        modtype::Rem,
        modtype::RemAssign,
        modtype::Num,
        modtype::Unsigned,
        modtype::Bounded,
        modtype::Zero,
        modtype::One,
        modtype::FromPrimitive,
        modtype::Inv,
        modtype::CheckedNeg,
        modtype::CheckedAdd,
        modtype::CheckedSub,
        modtype::CheckedMul,
        modtype::CheckedDiv,
        modtype::CheckedRem,
        modtype::Pow,
        modtype::Integer,
    )]
    #[modtype(
        modulus = "M::VALUE",
        std = "std",
        num_traits = "num::traits",
        num_integer = "num::integer",
        num_bigint = "num::bigint",
        from(InnerValue, BigUint, BigInt),
        debug(SingleTuple),
        neg(for_ref = true),
        add(for_ref = true),
        add_assign(for_ref = true),
        sub(for_ref = true),
        sub_assign(for_ref = true),
        mul(for_ref = true),
        mul_assign(for_ref = true),
        div(for_ref = true),
        div_assign(for_ref = true),
        rem(for_ref = true),
        rem_assign(for_ref = true),
        inv(for_ref = true),
        pow(for_ref = true)
    )]
    struct F_<M: ConstValue<Value = u32>> {
        #[modtype(value)]
        __value: u32,
        phantom: PhantomData<fn() -> M>,
    }
    assert_eq!(F(7) + F(13), F(3));
    assert_eq!(F(5) - F(11), F(11));
    assert_eq!(F(3), F(4) * F(5));
    assert_eq!(F(3) / F(4), F(5));
}

#[test]
fn test_modtype_derive() {
    run_modtype_derive();
}

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

// regex and lazy_static
// these codes were taken from examples on: https://docs.rs/regex/1.1.7/regex/
fn run_regex() -> UnitResult {
    use lazy_static::lazy_static;
    use regex::{Regex, RegexSet};

    // ...
    lazy_static! {
        static ref RE_YYYYMMDD: Regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
        static ref RE_SET: RegexSet =
            RegexSet::new(&[r"\w+", r"\d+", r"\pL+", r"foo", r"bar", r"barfoo", r"foobar",])
                .unwrap();
    }

    let text = "2012-03-14, 2013-01-01 and 2014-07-05";
    let mut iter = RE_YYYYMMDD.captures_iter(text);

    let mut cap;
    cap = iter.next().ok_or_else(|| "got none")?;
    assert_eq!((&cap[1], &cap[2], &cap[3]), ("2012", "03", "14"));
    cap = iter.next().ok_or_else(|| "got none")?;
    assert_eq!((&cap[1], &cap[2], &cap[3]), ("2013", "01", "01"));
    cap = iter.next().ok_or_else(|| "got none")?;
    assert_eq!((&cap[1], &cap[2], &cap[3]), ("2014", "07", "05"));

    // Iterate over and collect all of the matches.
    let matches: Vec<_> = RE_SET.matches("foobar").into_iter().collect();
    assert_eq!(matches, vec![0, 2, 3, 4, 6]);

    // You can also test whether a particular regex matched:
    let matches = RE_SET.matches("foobar");
    assert!(!matches.matched(5));
    assert!(matches.matched(6));

    Ok(())
}

#[test]
fn test_regex() -> UnitResult {
    run_regex()
}
