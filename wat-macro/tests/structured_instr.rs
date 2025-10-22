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
    let expected = expect![[
        r#"[Instr { unfolded_instr: If { label: Some("label"), block_type: None, then_block: [Instr { unfolded_instr: Nop, folded_instrs: [] }], else_block: [] }, folded_instrs: [] }]"#
    ]];
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
    let expected = expect![[
        r#"[Instr { unfolded_instr: If { label: Some("label"), block_type: None, then_block: [Instr { unfolded_instr: Nop, folded_instrs: [] }], else_block: [Instr { unfolded_instr: Nop, folded_instrs: [] }] }, folded_instrs: [] }]"#
    ]];
    expected.assert_eq(actual);
}

#[test]
pub fn if_label_type_then_else() {
    let actual = wat! {
       (if $lab
           (result i32)
           (const_i32 1)
           (then (const_i32 1))
           (else (const_i32 0)))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[
        r#"[Instr { unfolded_instr: If { label: Some("lab"), block_type: Some(Result(Num(I32))), then_block: [Instr { unfolded_instr: Const { ty: I32, val: 1 }, folded_instrs: [] }], else_block: [Instr { unfolded_instr: Const { ty: I32, val: 0 }, folded_instrs: [] }] }, folded_instrs: [Instr { unfolded_instr: Const { ty: I32, val: 1 }, folded_instrs: [] }] }]"#
    ]];
    expected.assert_eq(actual);
}

#[test]
pub fn if_no_label_type_then_else() {
    let fixnum_marker = 1i64;
    let actual = wat! {
        (if (result i32)
            (ref_test (ref i31) (local_get $n))
            (then (i32_and (const_i32 ,(fixnum_marker))
                           (i31_get_u (ref_cast (ref i31) (local_get $n)))))
            (else (const_i32 0)))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[
        r#"[Instr { unfolded_instr: If { label: None, block_type: Some(Result(Num(I32))), then_block: [Instr { unfolded_instr: I32And, folded_instrs: [Instr { unfolded_instr: Const { ty: I32, val: 1 }, folded_instrs: [] }, Instr { unfolded_instr: I31GetU, folded_instrs: [Instr { unfolded_instr: RefCast { ty: RefType { null: NonNullable, heap_type: Abs(I31) } }, folded_instrs: [Instr { unfolded_instr: LocalGet { name: "n" }, folded_instrs: [] }] }] }] }], else_block: [Instr { unfolded_instr: Const { ty: I32, val: 0 }, folded_instrs: [] }] }, folded_instrs: [Instr { unfolded_instr: RefTest { ty: RefType { null: NonNullable, heap_type: Abs(I31) } }, folded_instrs: [Instr { unfolded_instr: LocalGet { name: "n" }, folded_instrs: [] }] }] }]"#
    ]];
    expected.assert_eq(actual);
}
