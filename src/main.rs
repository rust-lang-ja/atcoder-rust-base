// -*- coding:utf-8-unix -*-

type UnitResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> UnitResult {
    run_proconio();
    run_ordered_float();
    run_ascii()?;
    run_bitset_fixed();
    run_permutohedron();
    run_superslice();
    run_itertools();
    run_rustc_hash();
    // run_smallvec();
    // run_im_rc();
    // run_num();
    run_rand_family()?;
    run_regex()?;
    // run_ndarray();
    // run_nalgebra();
    Ok(())
}

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

// proconio
#[proconio::fastout]
fn run_proconio() {
    use proconio::source::{line::LineSource, once::OnceSource, Source};
    use std::io::BufReader;

    run_proconio_for::<OnceSource<BufReader<&[u8]>>>();
    run_proconio_for::<LineSource<BufReader<&[u8]>>>();

    #[proconio::fastout]
    fn run_proconio_for<'a, T: Source<BufReader<&'a [u8]>> + From<&'a str>>() {
        use proconio::input;

        let source = T::from(
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
}

#[test]
fn test_proconio() {
    run_proconio();
}

// ordered-float
fn run_ordered_float() {
    use ordered_float::OrderedFloat;
    use rustc_hash::FxHasher;
    use std::f64::{INFINITY, NAN};
    use std::hash::{Hash, Hasher};

    let mut v = [
        8.20, -5.83, -0.21, 3.44, -7.12, 3.39, -0.72, -1.07, 9.36, NAN, 5.16, -2.81, 1.02, -8.67,
        5.77, -1.24, 0.44, 9.91, -7.06, INFINITY, -3.93, 5.82, 9.64, -8.04, -4.53,
    ]
    .iter()
    .map(|&n| OrderedFloat(n))
    .collect::<Vec<_>>();

    assert_eq!(v.iter().min(), Some(&OrderedFloat(-8.67)));
    assert_eq!(v.iter().max(), Some(&OrderedFloat(NAN)));

    v.sort_unstable();

    let size = v.len();
    assert_eq!(v[0], OrderedFloat(-8.67));
    assert_eq!(v[size - 2], OrderedFloat(INFINITY));
    assert_eq!(v[size - 1], OrderedFloat(NAN));

    let mut hasher = FxHasher::default();
    v[0].hash(&mut hasher);
    println!("hash for {} is {}", v[0], hasher.finish());

    v.pop(); // NAN
    v.pop(); // INFINITY

    let s = v.iter().map::<f64, _>(|&n| n.into()).sum::<f64>();
    assert!(10.91 < s && s < 10.92);
}

#[test]
fn test_ordered_float() {
    run_ordered_float();
}

// ascii
fn run_ascii() -> UnitResult {
    use ascii::AsciiString;
    // use ascii::{AsciiChar, AsciiStr, AsciiString};
    use std::str::FromStr;

    let s = AsciiString::from_str("2019-07-01")?;
    let mut ix = s.as_str().match_indices('-');
    let (i0, _) = ix.next().ok_or_else(|| "got none")?;
    let (i1, _) = ix.next().ok_or_else(|| "got none")?;

    assert_eq!(s[..i0].as_str(), "2019");
    assert_eq!(s[i0 + 1..i1].as_str(), "07");
    assert_eq!(s[i1 + 1..].as_str(), "01");

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
    use rand::prelude::*;
    use rand_distr::Uniform;

    let rng = StdRng::seed_from_u64(114_514);
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
    assert_eq!(ans, 13465);
}

#[test]
fn test_bitset_fixed() {
    run_bitset_fixed()
}

// permutohedron
fn run_permutohedron() {
    use permutohedron::Heap;

    let mut data = vec![1, 2, 3];

    let mut permutations = Heap::new(&mut data).collect::<Vec<_>>();
    assert_eq!(permutations.len(), 6);

    permutations.sort_unstable();
    assert_eq!(
        permutations,
        [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1]
        ]
    );
}

#[test]
fn test_permutohedron() {
    run_permutohedron();
}

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
fn run_rustc_hash() {
    use rustc_hash::FxHashMap;

    let mut map = [('c', "Cindy"), ('a', "Alice"), ('b', "Bob")]
        .iter()
        .map(|&(c, s)| (c, s.to_string()))
        .collect::<FxHashMap<_, _>>();
    map.entry('d').or_insert_with(|| "Denis".to_string());
    map.insert('a', "Alexa".to_string());
    assert_eq!(map.len(), 4);
}

#[test]
fn test_rustc_hash() {
    run_rustc_hash();
}

// smallvec

// im-rc

// num

// rand, rand_chacha, rang_pcg
fn run_rand_family() -> UnitResult {
    use rand::prelude::*;
    use rand_chacha::ChaChaRng;
    use rand_distr::{Normal, Uniform};
    use rand_pcg::Pcg64Mcg;

    macro_rules! test_mean {
        ($($rng:ident @ $distr:expr,)*) => {
            $(
                let mut rng = $rng::from_rng(thread_rng())?;
                let mean = calc_mean(&mut rng, &$distr);
                println!("{}: mean = {:.4}", stringify!($rng), mean);
                assert_eq!((mean * 10.0).round() as u32, 5);
            )*
        };
    }

    let distr_normal = Normal::new(0.5, 1.0).unwrap();
    let distr_uniform = Uniform::from(0.0..1.0);

    test_mean! {
        SmallRng  @ distr_uniform,
        ChaChaRng @ distr_uniform,
        Pcg64Mcg  @ distr_uniform,

        SmallRng  @ distr_normal,
        ChaChaRng @ distr_normal,
        Pcg64Mcg  @ distr_normal,
    }

    Ok(())
}

#[test]
fn test_rand_family() -> UnitResult {
    run_rand_family()
}

fn calc_mean<D: rand_distr::Distribution<f64>>(rng: &mut impl rand::Rng, distr: &D) -> f64 {
    use rand::Rng;

    const ITERATIONS: usize = 10000;

    // the stardard distribution for f64 generates a random rumber in interval [0, 1)
    rng.sample_iter::<f64, _>(distr)
        .take(ITERATIONS)
        .enumerate()
        // calculate the mean iteratively. https://stackoverflow.com/a/1934266
        .fold(0.0, |mean, (t, x)| mean + (x - mean) / (t + 1) as f64)
}

// regex and lazy_static
// these codes were taken from examples on: https://docs.rs/regex/1.1.7/regex/
#[allow(clippy::trivial_regex)]
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
