use expect_test::expect;
use wat_macro::wat;

#[test]
pub fn func() {
    let actual = wat! { (func $my_func) };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"Func { name: "my_func", exported: NotExported, type_use: None, params: [], results: [], locals: [], instrs: [] }"#]];
    expected.assert_eq(actual);
}

#[test]
pub fn func_full() {
    let actual = wat! {
        (func $my_func
            (param $a i32) (param $b (ref null $str))
            (result i32)
            (local $c i32)
            (const_i32 1))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"Func { name: "my_func", exported: NotExported, type_use: None, params: [Param { name: "a", ty: Num(I32) }, Param { name: "b", ty: Ref(RefType { null: Nullable, heap_type: TypeIdx("str") }) }], results: [Num(I32)], locals: [Local { name: "c", ty: Num(I32) }], instrs: [Instr { unfolded_instr: Const { ty: I32, val: 1 }, folded_instrs: [] }] }"#]];
    expected.assert_eq(actual);
}
