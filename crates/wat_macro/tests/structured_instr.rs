use expect_test::expect;
use wat_macro::wat;

#[test]
pub fn _loop() {
    let actual = wat! { (loop $for (nop)) };
    let actual = &format!("{:?}", actual);
    let expected = expect!["Instr { instr: Loop { label: \"for\" }, folded_instrs: [Instr { instr: Nop, folded_instrs: [] }] }"];
    expected.assert_eq(actual);

    assert!(true)
}
