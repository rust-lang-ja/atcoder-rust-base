use num_derive::{FromPrimitive, Num, NumCast, NumOps, One, ToPrimitive, Zero};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromPrimitive,
    ToPrimitive,
    Zero,
    One,
    Num,
    NumCast,
    NumOps,
)]
struct Weight(i32);

#[allow(clippy::eq_op)]
#[test]
fn check_ops() {
    let w1 = Weight(7);
    let w2 = Weight(4);

    assert_eq!(w1 + w2, Weight(11));
    assert_eq!(w1 - w2, Weight(3));
    assert_eq!(w1 * w2, Weight(28));
    assert_eq!(w1 / w2, Weight(1));
    assert_eq!(w1 % w2, Weight(3));
    assert!(w1 > w2);
    assert!(w2 < w1);
    assert!(w1 >= w1);
    assert!(w1 <= w1);
    assert!(w1 != w2);
    assert!(w1 == w1);
}
