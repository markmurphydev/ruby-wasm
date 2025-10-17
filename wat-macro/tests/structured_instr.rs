use expect_test::expect;
use wat_macro::wat;

#[test]
pub fn _loop() {
    // TODO `loop` needs to accept "no block_type"
    // let actual = wat! { (loop $for (nop)) };
    // let actual = &format!("{:?}", actual);
    // let expected = expect![[r#"[Instr { unfolded_instr: Loop { label: "for" }, folded_instrs: [Instr { unfolded_instr: Nop, folded_instrs: [] }] }]"#]];
    // expected.assert_eq(actual);
}

#[test]
pub fn if_label_then() {
    let actual = wat! { (if $label (then (nop)) ) };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"[Instr { unfolded_instr: If { label: Some("label"), block_type: None, then_block: [Instr { unfolded_instr: Nop, folded_instrs: [] }], else_block: [] }, folded_instrs: [] }]"#]];
    expected.assert_eq(actual);
}

#[test]
pub fn if_label_then_else() {
    let actual = wat! {
       (if $label
           (then (nop))
           (else (nop)))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"[Instr { unfolded_instr: If { label: Some("label"), block_type: None, then_block: [Instr { unfolded_instr: Nop, folded_instrs: [] }], else_block: [Instr { unfolded_instr: Nop, folded_instrs: [] }] }, folded_instrs: [] }]"#]];
    expected.assert_eq(actual);
}

#[test]
pub fn if_label_type_then_else() {
    let actual = wat! {
       (if $lab
           (result i32)
           (then (const_i32 1))
           (else (const_i32 0)))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"[Instr { unfolded_instr: If { label: Some("lab"), block_type: Some(Result(Num(I32))), then_block: [Instr { unfolded_instr: Const { ty: I32, val: 1 }, folded_instrs: [] }], else_block: [Instr { unfolded_instr: Const { ty: I32, val: 0 }, folded_instrs: [] }] }, folded_instrs: [] }]"#]];
    expected.assert_eq(actual);
}
