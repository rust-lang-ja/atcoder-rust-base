// https://atcoder.jp/contests/abc142/tasks/abc142_d

use defmac::defmac;
use primal::Sieve;

use std::cmp::max;
use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    defmac!(read => input.next().unwrap().parse().unwrap());

    let (a, b): (usize, usize) = (read!(), read!());

    let sieve = Sieve::new(num_integer::sqrt(max(a, b)));
    let bases = |k| -> HashSet<_> {
        sieve
            .factor(k)
            .unwrap()
            .into_iter()
            .map(|(p, _)| p)
            .collect()
    };
    println!("{}", (&bases(a) & &bases(b)).len() + 1);
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
