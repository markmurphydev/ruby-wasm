use expect_test::expect;
use proc_macro2::TokenStream;
use quote::quote;
use wat_macro::wat;

#[test]
pub fn global_immutable_nop() {
    let actual = wat! { (global $glob i32 (nop)) };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"Global { name: "glob", ty: GlobalType { mutable: Const, val_type: Num(I32) }, instr_seq: [Instr { unfolded_instr: Nop, folded_instrs: [] }] }"#]];
    expected.assert_eq(actual);
}

#[test]
pub fn global_mutable_const() {
    let actual = wat! { (global $true (mut i32) (const_i32 1)) };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"Global { name: "true", ty: GlobalType { mutable: Mutable, val_type: Num(I32) }, instr_seq: [Instr { unfolded_instr: Const { ty: I32, val: 1 }, folded_instrs: [] }] }"#]];
    expected.assert_eq(actual);
}

#[test]
pub fn array_new_fixed() {
    let actual = wat! {
        (global $empty_args
               (ref $arr_unitype)
               (array_new_fixed $arr_unitype 0))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"Global { name: "empty_args", ty: GlobalType { mutable: Const, val_type: Ref(RefType { null: NonNullable, heap_type: TypeIdx("arr_unitype") }) }, instr_seq: [Instr { unfolded_instr: ArrayNewFixed { type_idx: "arr_unitype", len: 0 }, folded_instrs: [] }] }"#]];
    expected.assert_eq(actual);
}

#[test]
pub fn global_ref_null() {
    let name = "asdf";
    let actual = wat! {
        (global ,(name)
                (mut (ref null eq))
                (ref_null eq))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"Global { name: "asdf", ty: GlobalType { mutable: Mutable, val_type: Ref(RefType { null: Nullable, heap_type: Abs(Eq) }) }, instr_seq: [Instr { unfolded_instr: RefNull { ty: Abs(Eq) }, folded_instrs: [] }] }"#]];
    expected.assert_eq(&actual);
}
