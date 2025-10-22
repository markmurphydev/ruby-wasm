use crate::func::Param;

#[derive(Debug, Clone)]
pub enum Mutable {
    Const,
    Mutable,
}

#[derive(Debug, Clone)]
pub struct GlobalType {
    pub mutable: Mutable,
    pub val_type: ValType,
}

#[derive(Debug, Clone)]
pub enum NumType {
    I32,
    I64,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum HeapType {
    Abs(AbsHeapType),
    TypeIdx(String),
}

#[derive(Debug, Clone)]
pub enum Nullable {
    NonNullable,
    Nullable,
}

#[derive(Debug, Clone)]
pub struct RefType {
    pub null: Nullable,
    pub heap_type: HeapType,
}

impl RefType {
    pub fn into_val_type(self) -> ValType {
        ValType::Ref(self)
    }
    pub fn into_storage_type(self) -> StorageType {
        StorageType::Val(self.into_val_type())
    }
}

#[derive(Debug, Clone)]
pub enum ValType {
    Num(NumType),
    Ref(RefType),
}

#[derive(Debug, Clone)]
pub enum BlockType {
    Result(ValType),
    TypeUse(String),
}

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct SubType {
    pub is_final: Final,
    pub supertypes: Vec<String>,
    pub comp_type: CompType,
}

#[derive(Debug, Clone)]
pub enum CompType {
    Struct(StructType),
    Array(ArrayType),
    Func(FuncType),
}

#[derive(Debug, Clone)]
pub struct StructType {
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: FieldType,
}

#[derive(Debug, Clone)]
pub struct FieldType {
    pub mutable: Mutable,
    pub ty: StorageType,
}

#[derive(Debug, Clone)]
pub enum StorageType {
    Val(ValType),
    Pack(PackType),
}

#[derive(Debug, Clone)]
pub enum PackType {
    I8,
    I16,
}

#[derive(Debug, Clone)]
pub struct ArrayType {
    pub field_type: FieldType,
}

#[derive(Debug, Clone)]
pub struct FuncType {
    pub params: Vec<Param>,
    pub results: Vec<ValType>,
}
