use procontest::*;

#[test]
fn sample1() {
    let res = do_test("sample1", include_str!("in1.txt"), include_str!("out1.txt"));
    assert_eq!(TestResult::Accepted, res, "{}", format(&res));
}

#[test]
fn sample2() {
    let res = do_test("sample2", include_str!("in2.txt"), include_str!("out2.txt"));
    assert_eq!(TestResult::Accepted, res, "{}", format(&res));
}

#[test]
fn sample3() {
    let res = do_test("sample3", include_str!("in3.txt"), include_str!("out3.txt"));
    assert_eq!(TestResult::Accepted, res, "{}", format(&res));
}
