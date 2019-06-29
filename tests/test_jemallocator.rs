// -*- coding:utf-8-unix -*-

use jemalloc_ctl;
use jemallocator;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

type UnitResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> UnitResult {
    do_allocate_heap()
}

#[test]
fn test_jemallocator() -> UnitResult {
    do_allocate_heap()
}

fn do_allocate_heap() -> UnitResult {
    use jemalloc_ctl::{epoch, stats};
    use rand::prelude::*;

    const SIZE: usize = 100000;

    let mut rng = SmallRng::from_rng(thread_rng())?;

    let v = rng
        .sample_iter(&rand::distributions::Standard)
        .take(SIZE)
        .collect::<Vec<usize>>();
    let v_byte_size = v.len() * std::mem::size_of::<usize>();

    // many statistics are cached and only updated when the epoch is advanced.
    epoch::advance().map_err(stringify)?;
    let allocated = stats::allocated::read().map_err(stringify)?;
    let resident = stats::resident::read().map_err(stringify)?;
    println!(
        "{} bytes used by a Vec<uzize> with len {}.",
        v_byte_size,
        v.len()
    );
    println!(
        "{} bytes allocated/{} bytes resident using jemalloc",
        allocated, resident
    );

    assert!(
        allocated >= v_byte_size,
        "allocated size ({} bytes) is smaller than the vector size ({} bytes).",
        allocated,
        v_byte_size,
    );

    // to prevent the compiler to optimize the vec out, read its value at
    // a random location.
    let i = rng.gen_range(0, SIZE);
    println!("v[{}] = {}", i, v[i]);

    Ok(())
}

fn stringify(x: impl ToString) -> String {
    x.to_string()
}
