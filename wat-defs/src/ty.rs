use crate::func::Param;

#[derive(Debug)]
pub enum Mutable {
    Const,
    Mutable,
}

#[derive(Debug)]
pub struct GlobalType {
    pub mutable: Mutable,
    pub val_type: ValType,
}

#[derive(Debug)]
pub enum NumType {
    I32,
}

#[derive(Debug)]
pub enum AbsHeapType {
    Func,
    Extern,
    Any,
    None,
    NoExtern,
    NoFunc,
    Eq,
    Struct,
    Array,
    I31,
    Exn,
    NoExn,
}

#[derive(Debug)]
pub enum HeapType {
    Abs(AbsHeapType),
    TypeIdx(String),
}

#[derive(Debug)]
pub enum Nullable {
    NonNullable,
    Nullable,
}

#[derive(Debug)]
pub struct RefType {
    pub null: Nullable,
    pub heap_type: HeapType,
}

#[derive(Debug)]
pub enum ValType {
    Num(NumType),
    Ref(RefType),
}

#[derive(Debug)]
pub enum BlockType {
    Result(ValType),
    TypeUse(String),
}

#[derive(Debug)]
pub enum Final {
    NotFinal,
    Final,
}

/// A composite type with an optional list of supertypes it matches.
///     Can be marked `final` to prevent further subtyping.
///
/// ```ebnf
/// <SUBTYPE> ::= (sub final? supertypes:<TYPEIDX>* <COMPTYPE>)
///             | (sub final <COMPTYPE>)                // Final subtype with no supertype
/// ```
#[derive(Debug)]
pub struct SubType {
    pub is_final: Final,
    pub supertypes: Vec<String>,
    pub comp_type: CompType,
}

#[derive(Debug)]
pub enum CompType {
    Struct(StructType),
    Array(ArrayType),
    Func(FuncType),
}

#[derive(Debug)]
pub struct StructType {
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub ty: FieldType,
}

#[derive(Debug)]
pub struct FieldType {
    pub mutable: Mutable,
    pub ty: StorageType,
}

#[derive(Debug)]
pub enum StorageType {
    Val(ValType),
    Pack(PackType),
}

#[derive(Debug)]
pub enum PackType {
    I8,
    I16,
}

#[derive(Debug)]
pub struct ArrayType {
    pub field_type: FieldType,
}

#[derive(Debug)]
pub struct FuncType {
    pub params: Vec<Param>,
    pub results: Vec<ValType>,
}
