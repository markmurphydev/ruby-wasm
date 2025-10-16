use expect_test::expect;
use wat_macro::wat;

#[test]
pub fn _loop() {
    let actual = wat! { (loop $for (nop)) };
    let actual = &format!("{:?}", actual);
    let expected = expect!["Instr { instr: Loop { label: \"for\" }, folded_instrs: [Instr { instr: Nop, folded_instrs: [] }] }"];
    expected.assert_eq(actual);
}

#[test]
pub fn _if() {
    let actual = wat! { (if $label (then (nop)) ) };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"Instr { instr: If { label: Some("label"), block_type: None, then_block: [Instr { instr: Nop, folded_instrs: [] }], else_block: [] }, folded_instrs: [] }"#]];
    expected.assert_eq(actual);

    let actual = wat! {
       (if $label
           (then (nop))
           (else (nop)))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"Instr { instr: If { label: Some("label"), block_type: None, then_block: [Instr { instr: Nop, folded_instrs: [] }], else_block: [Instr { instr: Nop, folded_instrs: [] }] }, folded_instrs: [] }"#]];
    expected.assert_eq(actual);

    let actual = wat! {
       (if $label
           (result i32)
           (then (nop))
           (else (nop)))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"Instr { instr: If { label: Some("label"), block_type: None, then_block: [Instr { instr: Nop, folded_instrs: [] }], else_block: [Instr { instr: Nop, folded_instrs: [] }] }, folded_instrs: [] }"#]];
    expected.assert_eq(actual);
}
