// https://atcoder.jp/contests/abc151/tasks/abc151_d

use ndarray::Array;
use proconio::input;
use proconio::marker::Bytes;
use smallvec::SmallVec;

use std::{iter, mem};

fn main() {
    input! {
        h: usize,
        w: usize,
        sss: [Bytes; h],
    }

    let maze = Array::from_shape_vec((h, w), itertools::concat(sss))
        .unwrap()
        .map(|&c| c == b'.');

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
                        .filter(|&p| mem::take(&mut unvisited[p]))
                        .collect();
                    !queue.is_empty()
                })
                .count()
        })
        .max()
        .unwrap();
    println!("{}", ans);
}
