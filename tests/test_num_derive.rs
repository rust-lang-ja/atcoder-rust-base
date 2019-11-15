use num_derive::{FromPrimitive, Num, NumCast, NumOps, One, ToPrimitive, Zero};

#[derive(
    Debug,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    FromPrimitive,
    ToPrimitive,
    One,
    Zero,
    Num,
    NumCast,
    NumOps,
)]
struct Weight(i32);

fn main() {
    let w1 = Weight(3);
    let w2 = Weight(4);
    println!("{:?}", w1 + w2); // => "Weight(7)"
}
