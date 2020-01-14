// https://atcoder.jp/contests/abc084/tasks/abc084_d

use itertools_num::ItertoolsNum as _;
use primal::Sieve;

use std::io::{self, BufWriter, Read, StdoutLock, Write as _};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read {
        ([$tt:tt])          => { read!([$tt; read!(usize)]) };
        ([$tt:tt; $n:expr]) => { (0..$n).map(|_| read!($tt)).collect::<Vec<_>>() };
        (($($tt:tt),+))     => { ($(read!($tt)),*) };
        ($ty:ty)            => { input.next().unwrap().parse::<$ty>().unwrap() };
    }

    let lrs = read!([(usize, usize)]);

    let max = lrs.iter().map(|&(_, r)| r).max().unwrap();
    let sieve = Sieve::new(max);
    let nums = (0..=max)
        .map(|k| u32::from(sieve.is_prime(k) && sieve.is_prime((k + 1) / 2)))
        .cumsum()
        .collect::<Vec<u32>>();

    buf_print(|stdout| {
        macro_rules! println { ($($tt:tt)*) => { writeln!(stdout, $($tt)*).unwrap() }; }

        for (l, r) in lrs {
            println!("{}", nums[r] - nums[l - 1]);
        }
    });
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}

fn buf_print(f: impl FnOnce(&mut BufWriter<StdoutLock>)) {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    f(&mut stdout);
    stdout.flush().unwrap();
}
