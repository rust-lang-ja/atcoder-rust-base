// https://atcoder.jp/contests/abc151/tasks/abc151_d

use ndarray::Array;
use smallvec::SmallVec;

use std::io::{self, Read};
use std::{iter, mem};

fn main() {
    let mut input = read_to_static(io::stdin()).split_whitespace();
    macro_rules! read {
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
        ({ Maze<$c:literal, ($h:expr, $w:expr)> }) => {
            Array::from_shape_vec(
                ($h, $w),
                itertools::concat((0..$h).map(|_| read!({ Row<$c> }))),
            )
            .unwrap()
        };
        ({ Row<$c:literal> }) => {
            read!({ Bytes }).into_iter().map(|c| c == $c).collect::<Vec<_>>()
        };
        ({ Bytes }) => {
            read!(String).into_bytes()
        };
    }

    let (h, w) = read!((usize, usize));
    let maze = read!({ Maze<b'.', (h, w)> });

    let neighbors = Array::from_shape_fn((h, w), |(i, j)| {
        let mut neighbors = SmallVec::<[_; 4]>::new();
        macro_rules! push((if $cond:expr => $pos:expr) => {
            if $cond && maze[$pos] {
                neighbors.push($pos);
            }
        });
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
            let mut queue = vec![start];
            let mut unvisited = maze.clone();
            unvisited[start] = false;

            iter::repeat(())
                .take_while(|_| {
                    queue = queue
                        .iter()
                        .flat_map(|&p| &neighbors[p])
                        .copied()
                        .filter(|&p| mem::replace(&mut unvisited[p], false))
                        .collect();
                    !queue.is_empty()
                })
                .count()
        })
        .max()
        .unwrap();
    println!("{}", ans);
}

fn read_to_static(mut source: impl Read) -> &'static str {
    let mut input = "".to_owned();
    source.read_to_string(&mut input).unwrap();
    Box::leak(input.into_boxed_str())
}
