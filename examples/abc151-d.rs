// https://atcoder.jp/contests/abc151/tasks/abc151_d

use ndarray::Array;
use smallvec::{smallvec, SmallVec};

use std::collections::VecDeque;
use std::io::{self, Read as _};
use std::iter;

fn main() {
    let mut input = "".to_owned();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read {
        (_maze<$c:literal, ($h:expr, $w:expr)>) => {
            Array::from_shape_vec(
                ($h, $w),
                (0..$h)
                    .fold(vec![], |mut acc, _| {
                        acc.extend(input.next().unwrap().bytes().map(|c| c == $c));
                        acc
                    }),
            )
            .unwrap()
        };
        ([$tt:tt])          => { read!([$tt; read!(usize)]) };
        ([$tt:tt; $n:expr]) => { (0..$n).map(|_| read!($tt)).collect::<Vec<_>>() };
        (($($tt:tt),+))     => { ($(read!($tt)),*) };
        ($ty:ty)            => { input.next().unwrap().parse::<$ty>().unwrap() };
    }

    let (h, w) = read!((usize, usize));
    let maze = read!(_maze<b'.', (h, w)>);

    let neighbors = Array::from_shape_fn((h, w), |(i, j)| -> SmallVec<[_; 4]> {
        let mut neighbors = smallvec![];
        macro_rules! push {
            (if $cond:expr => $pos:expr) => {
                if $cond && maze[$pos] {
                    neighbors.push($pos);
                }
            };
        }
        push!(if 0 < i     => (i - 1, j));
        push!(if i < h - 1 => (i + 1, j));
        push!(if 0 < j     => (i, j - 1));
        push!(if j < w - 1 => (i, j + 1));
        neighbors
    });

    let ans = (0..h)
        .flat_map(|i| (0..w).map(move |j| (i, j)))
        .filter(|&p| maze[p])
        .map(|start| {
            let mut longest = 0;
            let mut queue = iter::once((start, 0)).collect::<VecDeque<_>>();
            let mut unvisited = maze.clone();
            unvisited[start] = false;

            while let Some((pos, dist)) = queue.pop_front() {
                for &neighbor in &neighbors[pos] {
                    if unvisited[neighbor] {
                        unvisited[neighbor] = false;
                        longest = dist + 1;
                        queue.push_back((neighbor, longest));
                    }
                }
            }
            longest
        })
        .max()
        .unwrap();
    println!("{}", ans);
}
