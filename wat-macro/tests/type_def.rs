use expect_test::expect;
use wat_macro::wat;

#[test]
pub fn type_def() {
    let actual = wat! { (type $str (array i8)) };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"TypeDef { name: "str", ty: SubType { is_final: Final, supertypes: [], comp_type: Array(ArrayType { field_type: FieldType { mutable: Const, ty: Pack(I8) } }) } }"#]];
    expected.assert_eq(actual);
}

#[test]
pub fn struct_def() {
    let actual = wat! {
        (type $obj (struct (field $parent (mut (ref null $class)))))
    };
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"TypeDef { name: "obj", ty: SubType { is_final: Final, supertypes: [], comp_type: Struct(StructType { fields: [Field { name: "parent", ty: FieldType { mutable: Mutable, ty: Val(Ref(RefType { null: Nullable, heap_type: TypeIdx("class") })) } }] }) } }"#]];
    expected.assert_eq(actual);
}
