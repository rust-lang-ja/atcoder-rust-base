// https://atcoder.jp/contests/abc142/tasks/abc142_d

use defmac::defmac;
use primal::Sieve;

use std::cmp::max;
use std::collections::HashSet;
use std::io::{self, Read as _};

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    defmac!(read => input.next().unwrap().parse().unwrap());

    let (a, b): (usize, usize) = (read!(), read!());

    // サンプルケースでしか試してないので嘘かもしれない。

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
