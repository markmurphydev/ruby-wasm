use expect_test::expect;
use wat_macro::wat;

#[test]
pub fn global_immutable_nop() {
    let actual = wat! { (global $glob i32 (nop)) };
    let actual = &format!("{:?}", actual);
    let expected = expect![""];
    expected.assert_eq(actual);
}

#[test]
pub fn global_mutable_const() {
    let actual = wat! { (global $true (mut i32) (const_i32 1)) };
    let actual = &format!("{:?}", actual);
    let expected = expect![""];
    expected.assert_eq(actual);
}
