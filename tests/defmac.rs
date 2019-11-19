use defmac::defmac; // 2018 edition style

#[test]
fn incr() {
    let mut acc = 0;
    defmac!(incr => acc += 1);
    incr!();
    assert_eq!(acc, 1);
}
