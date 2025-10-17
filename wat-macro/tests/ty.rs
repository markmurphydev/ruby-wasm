use wat_defs::ty::StorageType;
use wat_defs::ty::NumType;
use expect_test::expect;
use wat_defs::ty::ValType;
use wat_macro::wat;

#[test]
pub fn ref_type_quasi_quote() {
    let actual = wat! { (ref ,("asdf".to_string())) };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"RefType { null: NonNullable, heap_type: TypeIdx("asdf") }"#]];
    expected.assert_eq(actual);
}

#[test]
pub fn type_name_quasi_quote() {
    let actual = wat! { (type ,("my_name".to_string()) (array (ref eq))) };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"TypeDef { name: "my_name", ty: SubType { is_final: Final, supertypes: [], comp_type: Array(ArrayType { field_type: FieldType { mutable: Const, ty: Val(Ref(RefType { null: NonNullable, heap_type: Abs(Eq) })) } }) } }"#]];
    expected.assert_eq(actual);
}

#[test]
pub fn storage_type_quasi_quote() {
    let storage_type = StorageType::Val(ValType::Num(NumType::I32));
    let actual = wat! { (type $x (array ,(storage_type))) };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"TypeDef { name: "x", ty: SubType { is_final: Final, supertypes: [], comp_type: Array(ArrayType { field_type: FieldType { mutable: Const, ty: Val(Num(I32)) } }) } }"#]];
    expected.assert_eq(actual);
}
