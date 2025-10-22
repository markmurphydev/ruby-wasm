use expect_test::expect;
use wat_macro::wat;

#[test]
pub fn func() {
    let actual = wat! { (func $my_func) };
    let actual = &format!("{:?}", actual);
    let expected = expect![[
        r#"Func { name: "my_func", exported: NotExported, type_use: None, params: [], results: [], locals: [], instrs: [] }"#
    ]];
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
    let expected = expect![[
        r#"Func { name: "my_func", exported: NotExported, type_use: None, params: [Param { name: "a", ty: Num(I32) }, Param { name: "b", ty: Ref(RefType { null: Nullable, heap_type: TypeIdx("str") }) }], results: [Num(I32)], locals: [Local { name: "c", ty: Num(I32) }], instrs: [Instr { unfolded_instr: Const { ty: I32, val: 1 }, folded_instrs: [] }] }"#
    ]];
    expected.assert_eq(actual);
}

#[test]
pub fn func_method() {
    let name = "method".to_string();
    let actual = wat! {
        (func ,(name)
            (type $method)
            (param $self (ref $obj))
            (param $args (ref $arr_unitype))
            (result (ref eq)))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[
        r#"Func { name: "method", exported: NotExported, type_use: Some("method"), params: [Param { name: "self", ty: Ref(RefType { null: NonNullable, heap_type: TypeIdx("obj") }) }, Param { name: "args", ty: Ref(RefType { null: NonNullable, heap_type: TypeIdx("arr_unitype") }) }], results: [Ref(RefType { null: NonNullable, heap_type: Abs(Eq) })], locals: [], instrs: [] }"#
    ]];
    expected.assert_eq(actual);
}

#[test]
pub fn func_method_with_locals() {
    let name = "method".to_string();
    let local_name = "n".to_string();
    let actual = wat! {
        (func ,(name)
            (type $method)
            (param $self (ref $obj))
            (param $args (ref $arr_unitype))
            (local ,(local_name) (ref eq))
            (result (ref eq)))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[
        r#"Func { name: "method", exported: NotExported, type_use: Some("method"), params: [Param { name: "self", ty: Ref(RefType { null: NonNullable, heap_type: TypeIdx("obj") }) }, Param { name: "args", ty: Ref(RefType { null: NonNullable, heap_type: TypeIdx("arr_unitype") }) }], results: [], locals: [Local { name: "n", ty: Ref(RefType { null: NonNullable, heap_type: Abs(Eq) }) }], instrs: [] }"#
    ]];
    expected.assert_eq(actual);
}
