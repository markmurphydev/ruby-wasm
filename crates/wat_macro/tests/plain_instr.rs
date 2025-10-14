use expect_test::expect;
use wat_macro::wat;

#[test]
pub fn nop() {
    let actual = wat!{ (nop) };
    let actual = &format!("{:?}", actual);
    let expected = expect!["Instr { instr: Nop, folded_instrs: [] }"];
    expected.assert_eq(actual);
}